# Feature Start Workflow

**Trigger:** User commands "START #<ISSUE_NUMBER>"

**Pre-requisites:** `gh` CLI installed and authenticated.

## Step 1: Context Gathering

1. Read the issue details using `gh issue view <ISSUE_NUMBER>`.
2. Analyze `docs/project/architecture.md` and `docs/project_brief.md` to understand the impact.

## Step 2: Branch Creation (PowerShell)

Execute the following commands strictly in the terminal.

*Note: Replace `<ISSUE_NUMBER>` with the actual ID and `<SLUG>` with a short, kebab-case description.*

```powershell
git checkout main;
git pull origin main;
$issueId = "<ISSUE_NUMBER>";
$desc = "<SLUG>"; 
# Example: "feat/issue-42-fix-login"
git checkout -b "feat/issue-$issueId-$desc";
```

## Step 3: Documentation Update & Initial Commit

1. Update `docs/project/active-context.md`:
   - Set "Current Focus" to the Issue Title.
   - Add the Issue ID to "Active Problems" or "Recent Decisions".

2. Execute immediately:

**Git Commit Rule:** Use standard quotes. The system handles UTF-8 correctly (core.quotepath false is set).

- Do NOT use Here-Strings for git commit.

```powershell
git add docs/project/active-context.md;
git commit -m "chore(docs): start work on issue #<ISSUE_NUMBER>";
git push -u origin HEAD;
```

## Step 4: Create Linked Pull Request

Create a Pull Request with appropriate labels.

**PowerShell Rule for gh:**

- ALWAYS use Here-Strings (`@"..."@`) for Title and Body. This protects special characters when passing them to the CLI.
- Read the issue labels using `gh issue view` or assign manual ones.

```powershell
# Get issue labels to inherit them
$issueLabels = (gh issue view <ISSUE_NUMBER> --json labels | ConvertFrom-Json).labels.name -join ","
# If empty, use convention: "task,backend,fase-2"

# Use Here-Strings (@"..."@) for safety
$prTitle = @"
feat: implementación de búsqueda
"@

$prBody = @"
Closes #<ISSUE_NUMBER>
"@

gh pr create --title "$prTitle" --body "$prBody" --label "$issueLabels";
```

## Step 5: Execution Plan

1. Propose a mini-plan of 3-4 steps to solve the issue.
2. Ask the user for confirmation before writing the actual feature code.
