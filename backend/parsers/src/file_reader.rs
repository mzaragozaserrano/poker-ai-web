//! Módulo de lectura eficiente de archivos de historial.
//!
//! Implementa estrategias optimizadas de lectura según el tamaño del archivo:
//! - Archivos pequeños (< 10MB): Carga completa en memoria con `std::fs::read`
//! - Archivos grandes (>= 10MB): Lectura buffereada con `BufReader` de 64KB
//!
//! Todas las operaciones retornan bytes (`Vec<u8>`) para evitar conversiones
//! UTF-8 innecesarias en el hot path del parser.

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

/// Tamaño límite para decidir estrategia de lectura (10MB).
const SMALL_FILE_THRESHOLD: u64 = 10 * 1024 * 1024;

/// Tamaño del buffer para archivos grandes (64KB).
/// Optimizado para el tamaño de caché L3 del Ryzen 3800X.
const BUFFER_SIZE: usize = 64 * 1024;

/// Resultado de lectura de archivo.
#[derive(Debug)]
pub struct FileContent {
    /// Contenido del archivo en bytes.
    pub bytes: Vec<u8>,
    /// Tamaño del archivo en bytes.
    pub size: u64,
    /// Si se usó lectura optimizada (true) o buffereada (false).
    pub fast_path: bool,
}

/// Lee un archivo de historial usando la estrategia óptima según su tamaño.
///
/// # Estrategias
///
/// - **Archivos pequeños (< 10MB)**: Usa `std::fs::read()` para cargar todo
///   el contenido en memoria de una vez. Esto es más rápido para archivos
///   pequeños ya que evita overhead de buffering.
///
/// - **Archivos grandes (>= 10MB)**: Usa `BufReader` con buffer de 64KB
///   para evitar saturar la memoria con archivos masivos (> 100MB).
///
/// # Errores
///
/// Retorna error si:
/// - El archivo no existe
/// - No hay permisos de lectura
/// - Ocurre un error de I/O durante la lectura
///
/// # Ejemplo
///
/// ```no_run
/// use poker_parsers::file_reader::read_file_optimized;
///
/// let content = read_file_optimized("history.txt")?;
/// println!("Leídos {} bytes (fast path: {})", content.size, content.fast_path);
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn read_file_optimized<P: AsRef<Path>>(path: P) -> io::Result<FileContent> {
    let path = path.as_ref();
    let metadata = std::fs::metadata(path)?;
    let size = metadata.len();

    if size < SMALL_FILE_THRESHOLD {
        // Fast path: Carga completa en memoria
        let bytes = std::fs::read(path)?;
        Ok(FileContent {
            bytes,
            size,
            fast_path: true,
        })
    } else {
        // Slow path: Lectura buffereada para archivos grandes
        read_file_buffered(path, size)
    }
}

/// Lee un archivo grande usando `BufReader` con buffer de 64KB.
fn read_file_buffered<P: AsRef<Path>>(path: P, size: u64) -> io::Result<FileContent> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);
    let mut bytes = Vec::with_capacity(size as usize);

    reader.read_to_end(&mut bytes)?;

    Ok(FileContent {
        bytes,
        size,
        fast_path: false,
    })
}

/// Lee un archivo línea por línea de forma eficiente.
///
/// Útil para procesamiento streaming de archivos muy grandes
/// donde no queremos cargar todo en memoria.
///
/// # Ejemplo
///
/// ```no_run
/// use poker_parsers::file_reader::read_lines;
///
/// for line_result in read_lines("history.txt")? {
///     let line = line_result?;
///     println!("{}", line);
/// }
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(BUFFER_SIZE, file);
    Ok(reader.lines())
}

/// Lee un archivo y retorna un iterador sobre líneas como bytes.
///
/// Esta es la opción más eficiente cuando no necesitamos validación UTF-8
/// y queremos trabajar directamente con bytes en el parser.
///
/// # Ejemplo
///
/// ```no_run
/// use poker_parsers::file_reader::read_lines_bytes;
///
/// for line_result in read_lines_bytes("history.txt")? {
///     let line_bytes = line_result?;
///     // Procesar bytes directamente sin conversión UTF-8
/// }
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn read_lines_bytes<P: AsRef<Path>>(
    path: P,
) -> io::Result<impl Iterator<Item = io::Result<Vec<u8>>>> {
    let file = File::open(path)?;
    let reader = BufReader::with_capacity(BUFFER_SIZE, file);

    Ok(ByteLineIterator { reader })
}

/// Iterador que lee líneas como bytes sin validación UTF-8.
struct ByteLineIterator<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read> Iterator for ByteLineIterator<R> {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = Vec::new();
        match self.reader.read_until(b'\n', &mut line) {
            Ok(0) => None, // EOF
            Ok(_) => {
                // Remover \n o \r\n del final
                if line.ends_with(&[b'\n']) {
                    line.pop();
                    if line.ends_with(&[b'\r']) {
                        line.pop();
                    }
                }
                Some(Ok(line))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_small_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "Winamax Poker - Test\nLine 2\nLine 3";
        temp_file.write_all(content.as_bytes()).unwrap();

        let result = read_file_optimized(temp_file.path()).unwrap();
        assert_eq!(result.bytes, content.as_bytes());
        assert!(result.fast_path, "Should use fast path for small files");
        assert_eq!(result.size, content.len() as u64);
    }

    #[test]
    fn test_read_lines_bytes() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Line 1\nLine 2\nLine 3").unwrap();

        let lines: Vec<Vec<u8>> = read_lines_bytes(temp_file.path())
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], b"Line 1");
        assert_eq!(lines[1], b"Line 2");
        assert_eq!(lines[2], b"Line 3");
    }

    #[test]
    fn test_read_lines_bytes_with_crlf() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Line 1\r\nLine 2\r\n").unwrap();

        let lines: Vec<Vec<u8>> = read_lines_bytes(temp_file.path())
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], b"Line 1");
        assert_eq!(lines[1], b"Line 2");
    }

    #[test]
    fn test_file_not_found() {
        let result = read_file_optimized("nonexistent_file.txt");
        assert!(result.is_err());
    }
}
