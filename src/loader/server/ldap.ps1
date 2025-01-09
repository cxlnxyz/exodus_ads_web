param (
    [string]$username,
    [string]$password
)

$ldapQuery = "(&(objectClass=user)(sAMAccountName=$username))"

# Define the properties to retrieve
$properties = @("sAMAccountName", "displayName", "mail")

# Perform the LDAP query
$securePassword = ConvertTo-SecureString $password -AsPlainText -Force
$credential = New-Object System.Management.Automation.PSCredential ($username, $securePassword)
$users = Get-ADUser -Filter $ldapQuery -Credential $credential -Property $properties

if ($users) {
    Write-Output "Login successful"
} else {
    Write-Output "Invalid credentials"
}