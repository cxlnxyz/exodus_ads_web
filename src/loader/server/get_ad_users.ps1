try {
    $users = Get-ADUser -Filter * -Property SamAccountName, Name, EmailAddress, Enabled
    $userList = @()
    foreach ($user in $users) {
        $userInfo = @{
            SamAccountName = $user.SamAccountName
            Name = $user.Name
            EmailAddress = $user.EmailAddress
            Enabled = $user.Enabled
        }
        $userList += $userInfo
    }
    $userList | ConvertTo-Json
    exit 0
} catch {
    Write-Output "Failed to fetch users"
    exit 1
}