#!/bin/bash
echo --% >/dev/null;: ' | out-null
<#'

# FOR UBUNTU
# -----------------------------------------------------------
function UbuntuRemove {
    choice1="Remove CURL and GIT"
    choice2="Remove SplashKit Manager and dependencies"
    choice3="Remove Visual Studio Code"
    choice4="Remove .Net 6 LTS Core SDK"

    choices=$(
        zenity --list --checklist --column "Select" --column "Uninstall" --width=400 --height=300 \
            TRUE "${choice1}" \
            TRUE "${choice2}" \
            TRUE "${choice3}" \
            TRUE "${choice4}")

    # Remove CURL and GIT
    if ([[ "${choices}" =~ "${choice1}" ]]); then
        sudo apt-get remove curl git -y;
    fi

    # Remove SplashKit Manager and dependencies
    if [[ "${choices}" =~ "${choice2}" ]]; then
        rm -rf /home/$USER/.splashkit
    fi

    # Remove Visual Studio Code
    if [[ "${choices}" =~ "${choice3}" ]]; then
        sudo snap remove code;
    fi

    # Remove .NET 6 LTS Core SDK
    if [[ "${choices}" =~ "${choice4}" ]]; then
        sudo apt-get remove -y dotnet-host;
    fi

    zenity --info --text="Uninstall  Complete"
}

# FOR MacOS
# -----------------------------------------------------------
function MacRemove {
	read -p "Remove xcode-select (y/n)? " removeXcodeSelect
	read -p "Remove SplashKit SDK (y/n)? " removeSplashKitSDK
	read -p "Remove .NET SDK using brew (y/n)? " removeDotNet
	read -p "Remove VS Code using brew (y/n)? " removeVSCode
	read -p "Remove Homebrew (y/n)? " removeHomebrew

	# Uninstall xcode-select
	if [[ "$removeXcodeSelect" =~ ^([yY][eE][sS]|[yY]) ]]; then
		echo "Removing xcode-select ..."
		sudo rm -rf /Library/Developer/CommandLineTools		
	fi

	# Uninstall SplashKit SDK
	if [[ "$removeSplashKitSDK" =~ ^([yY][eE][sS]|[yY]) ]]; then
		echo "Removing SplashKit SDK ...";
		rm -rf /Users/$USER/.splashkit		
	fi

	# Uninstall dotnet
	if [[ "$removeDotNet" =~ ^([yY][eE][sS]|[yY]) ]]; then
		echo "Removing .Net SDK ...";
		brew uninstall dotnet;	
	fi

	# Uninstall VS Code
	if [[ "$removeVSCode" =~ ^([yY][eE][sS]|[yY]) ]]; then
		echo "Removing VS Code ...";
		brew uninstall visual-studio-code;	
	fi


	# Uninstall Homebrew package manager
	if [[ "$removeHomebrew" =~ ^([yY][eE][sS]|[yY]) ]]; then
		echo "Removing Homebrew ...";
		sudo yes | /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/uninstall.sh)"
	fi

    echo "Uninstallation Complete!"
}

# Check if Linux or Darwin
uname=$(uname)
case "$uname" in
	(*Linux*) UbuntuRemove; ;;
	(*Darwin*) MacRemove; ;;
	(*) echo "ERROR"; ;;
esac;

exit #>

#FOR WINDOWS
# -----------------------------------------------------------
[reflection.assembly]::loadwithpartialname("System.Windows.Forms") | Out-Null
[reflection.assembly]::loadwithpartialname("System.Drawing") | Out-Null

$form = New-Object System.Windows.Forms.Form
$button = New-Object System.Windows.Forms.Button
$checkbox1 = New-Object System.Windows.Forms.CheckBox
$checkbox2 = New-Object System.Windows.Forms.CheckBox
$checkbox3 = New-Object System.Windows.Forms.CheckBox
$InitialFormWindowState = New-Object System.Windows.Forms.FormWindowState

$handler_button_Click= 
{
    if ($checkbox1.Checked) {
        $checkbox1.ForeColor = 'red'
        $checkbox1.Text = "MSYS2, SplashKit, Git, and GCC (Removing...)"
        & "$env:LOCALAPPDATA\Microsoft\WindowsApps\winget" uninstall msys2
        $checkbox1.ForeColor = 'green'
        $checkbox1.Text = "MSYS2, SplashKit, Git, and GCC (Removed)"
    }

    if ($checkbox2.Checked) {
        $checkbox2.ForeColor = 'red'
        $checkbox2.Text = "VS Code (Removing...)"
        & "$env:LOCALAPPDATA\Microsoft\WindowsApps\winget" uninstall Microsoft.VisualStudioCode
        $checkbox2.ForeColor = 'green'
        $checkbox2.Text = "VS Code (Removed)"
    }

    if ($checkbox3.Checked) {
        $checkbox3.ForeColor = 'red'
        $checkbox3.Text = ".Net 6 LTS Core SDK (Removing...)" 
        & "$env:LOCALAPPDATA\Microsoft\WindowsApps\winget" uninstall Microsoft.DotNet.SDK.6
        $checkbox3.ForeColor = 'green'
        $checkbox3.Text = ".Net 6 LTS Core SDK (Removed)"
    }

    Start-Sleep 1
    $form.Close() | Out-Null
}

$form.Text = "Uninstall Tools"
$form.Name = "form"
$form.Font = New-Object System.Drawing.Font("Arial",11)
$form.DataBindings.DefaultDataSourceUpdateMode = 0
$System_Drawing_Size = New-Object System.Drawing.Size
$System_Drawing_Size.Width = 450
$System_Drawing_Size.Height = 200
$form.ClientSize = $System_Drawing_Size

$button.TabIndex = 6
$button.Text = "Uninstall"
$button.Name = "button"
$System_Drawing_Size = New-Object System.Drawing.Size
$System_Drawing_Size.Width = 80
$System_Drawing_Size.Height = 40
$button.Size = $System_Drawing_Size
$button.UseVisualStyleBackColor = $True

$System_Drawing_Point = New-Object System.Drawing.Point
$System_Drawing_Point.X = 25
$System_Drawing_Point.Y = 120
$button.Location = $System_Drawing_Point
$button.DataBindings.DefaultDataSourceUpdateMode = 0
$button.add_Click($handler_button_Click)

$form.Controls.Add($button)

$System_Drawing_Size = New-Object System.Drawing.Size
$System_Drawing_Size.Width = 400
$System_Drawing_Size.Height = 30

$checkbox1.UseVisualStyleBackColor = $True
$checkbox1.Size = $System_Drawing_Size
$checkbox1.TabIndex = 1
$checkbox1.Text = "Remove MSYS2, SplashKit, Git, and GCC using Winget"
$System_Drawing_Point = New-Object System.Drawing.Point
$System_Drawing_Point.X = 25
$System_Drawing_Point.Y = 15
$checkbox1.Location = $System_Drawing_Point
$checkbox1.DataBindings.DefaultDataSourceUpdateMode = 0
$checkbox1.Name = "checkbox1"

$form.Controls.Add($checkbox1)

$checkbox2.UseVisualStyleBackColor = $True
$checkbox2.Size = $System_Drawing_Size
$checkbox2.TabIndex = 2
$checkbox2.Text = "Remove VS Code using Winget"
$System_Drawing_Point = New-Object System.Drawing.Point
$System_Drawing_Point.X = 25
$System_Drawing_Point.Y = 45
$checkbox2.Location = $System_Drawing_Point
$checkbox2.DataBindings.DefaultDataSourceUpdateMode = 0
$checkbox2.Name = "checkbox2"

$form.Controls.Add($checkbox2)

$checkbox3.UseVisualStyleBackColor = $True
$checkbox3.Size = $System_Drawing_Size
$checkbox3.TabIndex = 4
$checkbox3.Text = "Remove .Net 6 LTS Core SDK using Winget"
$System_Drawing_Point = New-Object System.Drawing.Point
$System_Drawing_Point.X = 25
$System_Drawing_Point.Y = 75
$checkbox3.Location = $System_Drawing_Point
$checkbox3.DataBindings.DefaultDataSourceUpdateMode = 0
$checkbox3.Name = "checkbox3"

$form.Controls.Add($checkbox3)

$InitialFormWindowState = $form.WindowState
$form.ShowDialog() | Out-Null
