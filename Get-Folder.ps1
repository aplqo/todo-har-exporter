[CmdletBinding()]
param (
    [Parameter()]
    [String]
    $Path
)

function Read-Json([String]$Path) {
    return Get-Content -Raw -Path $Path | ConvertFrom-Json
}
function Get-ItemById([System.Array]$Array, [string]$Id) {
    return $Array.Where( { $_.Id -eq $Id }).Item(0)
}
function Write-File([string]$Path, [String]$Content) {
    if (Test-Path -Path $Path) {
        return
    }
    $dir = Split-Path -Path $Path -Parent
    if (-not (Test-Path -Path $dir)) {
        New-Item -ItemType Directory -ErrorAction SilentlyContinue -Path $dir
    }
    Out-File -InputObject $Content -FilePath $Path -NoClobber
}

$folders = (Read-Json -Path .\json\taskfolders.json).Value
$groups = (Read-Json -Path .\json\foldergroups.json).Value

function Get-Folder([String]$Url, [String]$Content) {
    Write-Host -Object "Converting request to $Url"
    if (-not ($Url -match '.+taskfolders/(.+)/tasks\?.+' )) {
        Write-Host -Object "Not contain folder"
        return
    }

    $folder = Get-ItemById -Array $folders -Id $Matches[1]
    Write-Host "Name: $($folder.Name)"

    if ($folder.ParentFolderGroupId) {
        $group = (Get-ItemById -Array $groups -Id $folder.ParentFolderGroupId).Name
        Write-Host "Group: $group"
        Write-File -Path ".\json\taskfolders\$group\$($folder.Name).json" -Content $Content
    }
    else {
        Write-File -Path ".\json\taskfolders\$($folder.Name).json" -Content $Content
    }
}

$data = (Get-Content -Raw .\to-do.live.com.har | ConvertFrom-Json).log.entries

foreach ($i in $data) {
    Get-Folder -Url $i.request.url -Content $i.response.content.text
}