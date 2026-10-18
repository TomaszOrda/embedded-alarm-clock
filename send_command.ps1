param(
    [Parameter(Mandatory=$true)]
    [string]$Command,
    [string]$ComPort,
    [int]$BaudRate = 115200,
    [int]$WaitMs = 300
)

# --- find port automatically if not given ---
if (-not $ComPort) {
    $dev = Get-CimInstance Win32_SerialPort |
        Where-Object { $_.Description -match 'USB.*(JTAG|Serial|UART)' -or $_.PNPDeviceID -match 'VID_303A' } |
        Select-Object -First 1
    if ($dev) {
        $ComPort = $dev.DeviceID
    } else {
        # fallback: any serial port at all
        $names = [System.IO.Ports.SerialPort]::GetPortNames()
        if ($names.Count -eq 0) {
            Write-Error "No serial ports found. Is the board plugged in?"
            exit 1
        }
        $ComPort = $names | Select-Object -First 1
    }
}

# --- special command: set-time sends current PC datetime (ISO) after a space ---
if ($Command -eq "set-time") {
    $Command = "set-time " + (Get-Date).ToString("yyyy-MM-ddTHH:mm:ss")
    Write-Host "sending: $Command"
}

Write-Host "using port: $ComPort"

$port = New-Object System.IO.Ports.SerialPort($ComPort, $BaudRate, [System.IO.Ports.Parity]::None, 8, [System.IO.Ports.StopBits]::One)
$port.ReadTimeout = 100

try {
    $port.Open()
    $port.WriteLine($Command)
    $deadline = [DateTime]::UtcNow.AddMilliseconds($WaitMs)
    $out = ""
    while ([DateTime]::UtcNow -lt $deadline) {
        try   { $out += $port.ReadExisting() }
        catch {}
        Start-Sleep -Milliseconds 50
    }
    Write-Host $out
}
finally {
    $port.Close()
}
