# Report Bug Workflow

**Trigger:** User commands "REPORT BUG {TITLE}" or just "BUG {TITLE}"

**Pre-requisites:** `gh` CLI installed.

## Step 1: Information Gathering

The agent must ensure the bug report is complete.

1. **Analyze the request.** If the user only provided a title, ASK for:
   - **Description:** What is happening?
   - **Reproduction Steps:** How can we trigger it?
   - **Context:** Browser/OS or specific Situation ID (if applicable).
   
   *Wait for user response before proceeding.*

## Step 2: Content Preparation & Sanitization

**Context: LOCAL PowerShell + gh cli**

1. **Consult References:**
   - Read `.github/docs/labels_convention.md` for label structure.

2. **Prepare Strings:**
   - ALWAYS use Here-Strings (`@"..."@`) for Title and Body.
   - This ensures PowerShell passes UTF-8 characters (tildes, ñ) correctly to the CLI.

3. **Determine Labels:**
   - Based on the bug context, assign labels following the convention:
     - **Type:** `bug` (mandatory for bugs)
     - **Technology:** `backend`, `frontend`, `database`, `devops`, or `testing`
     - **Phase:** `fase-1`, `fase-2`, `fase-3`, or `fase-4` (check `docs/project/roadmap.md` for current phase)

## Step 3: Execution (PowerShell)

Construct the command using Here-Strings.

*Note: Replace placeholders with actual content. Labels format: "bug,backend,fase-2" (comma-separated).*

```powershell
# Use Here-Strings (@"..."@) to safely handle special characters
$Title = @"
Error en validación de entrada
"@

$Body = @"
## Descripción
Describe el error aquí.

## Pasos para reproducir
1. Paso uno
2. Paso dos

## Contexto
Sistema operativo: Windows
Navegador: Chrome
"@

$Labels = "bug,backend,fase-2"

gh issue create --title "$Title" --body "$Body" --label "$Labels";
```

## Step 4: Documentation Update

1. Update `docs/project/active-context.md`:
   - Mention that a new bug has been reported (include the Issue URL or ID returned by the command).

2. Ask the user if they want to switch context to fix it immediately ("FIX #ID").
