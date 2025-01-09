param (
    [string]$username,
    [string]$password
)

$securePassword = ConvertTo-SecureString $password -AsPlainText -Force
$credential = New-Object System.Management.Automation.PSCredential ($username, $securePassword)

try {
    $null = (New-Object System.DirectoryServices.DirectorySearcher).FindOne()
    Write-Output "Login successful"
    exit 0
} catch {
    Write-Output "Login failed"
    exit 1
}