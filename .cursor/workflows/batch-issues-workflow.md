# Batch Issue Creation Workflow

**Trigger:** User commands "BATCH PHASE <PHASE_NUMBER>" (e.g., "BATCH PHASE 2")

**Pre-requisites:** `gh` CLI installed. `docs/project/roadmap.md` exists.

## Step 1: Validar/Crear project_labels.json

**IMPORTANTE:** Este paso verifica que existe la configuración de etiquetas.

1. Verificar que existe `.github/docs/project_labels.json`
   - **Si NO existe:**
     - **EJECUTAR:** `.\.github\scripts\Sync-ProjectLabels.ps1`
     - El script creará el archivo automáticamente y sincronizará con GitHub.
   - **Si YA existe:**
     - **NO ejecutar** el script de sincronización.
     - Solo usar el archivo para consultar etiquetas.

**Nota:** El usuario puede ejecutar manualmente la sincronización cuando lo desee:

```powershell
.\.github\scripts\Sync-ProjectLabels.ps1
```

## Step 2: Roadmap Analysis

1. Read `docs/project/roadmap.md`.
2. Locate the section corresponding to the requested Phase.
3. Extract all unchecked tasks (`- [ ]`) from that phase.
4. If a task has sub-tasks (indented), treat the parent as an "Epic" (or main issue) and sub-tasks as details in the Body.

## Step 3: Update Automation Script

You must update the content of the `$issues` array inside `.github/scripts/New-BatchIssues.ps1`.

**Instructions for the Agent:**

1. **String Format:** ALWAYS use Here-Strings (`@"..."@`) for Title and Body.
   - You can write UTF-8 characters (ñ, tildes) directly inside the Here-String.
   - Do NOT use subexpressions or hex codes.

2. **Assign Labels:** For each issue, assign labels following the convention:
   - **Type:** `task`, `bug`, or `documentation`
   - **Technology:** `backend`, `frontend`, `database`, `devops`, or `testing`
   - **Phase:** `fase-1`, `fase-2`, ... (based on roadmap)

**Example of code to inject:**

```powershell
$issues = @(
    # Correct Pattern: Here-Strings + Direct UTF-8
    @{ 
        Title = @"
UI: Botón de Pánico (Añadir)
"@
        Body = @"
Crear acción rápida para detener el proceso.
"@
        Labels = "task,frontend,fase-3" 
    }
)
```

## Step 4: User Confirmation

1. Show the user the list of titles you are about to create.
2. **STOP** and ask: "Do these issues look correct? Should I execute the script?"

## Step 5: Execution (PowerShell)

Once confirmed, execute the script to push issues to GitHub:

```powershell
.\.github\scripts\New-BatchIssues.ps1
```

## Step 6: Documentation Update

Update `docs/project/active-context.md`:

1. Note that Phase <X> has started.
2. List the newly created issues (if possible, ask user to run `gh issue list` to get IDs).
