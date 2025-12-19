# Windows & GitHub CLI Encoding Guide

Since the environment variables `LESSCHARSET=utf-8` and `LANG=C.UTF-8` are set in Windows, we can use a simplified approach without complex escape codes.

## 1. PowerShell & GitHub CLI (`gh`)

To ensure `gh` receives accents and special characters correctly on Windows PowerShell, **ALWAYS** use variables defined with Here-Strings (`@"..."@`).

### Correct Pattern (Safe & Clean)

Use this pattern for Issues and PRs:

```powershell
$Title = @"
Corrección de validación en el menú
"@

$Body = @"
Se ha corregido el error donde la 'ñ' no se mostraba bien.
Además, se optimizó la carga de datos.
"@

# The CLI handles the variables correctly when passed this way
gh issue create --title $Title --body $Body --label "bug,frontend"
```

### Avoid These Patterns

- **DO NOT** use `[char]0x00F3` or subexpressions like `validaci$([char]0x00F3)n`. It is no longer necessary.
- **DO NOT** pass multi-line strings directly without variables if they contain special characters.

## 2. Git Commits

Standard quotes work fine because we have set `git config core.quotepath false` globally.

```powershell
# Correct
git commit -m "fix: corrección de tildes y caracteres especiales"
```

## 3. Troubleshooting

If you ever see strange characters in the terminal output (visual only), run this command once in that session:

```powershell
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
```
