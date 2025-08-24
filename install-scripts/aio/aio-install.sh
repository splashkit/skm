#!/bin/bash
echo --% >/dev/null;: ' | out-null
<#'

# FOR UBUNTU
# -----------------------------------------------------------
function UbuntuInstall {
    choice1="CURL and GIT using apt Package"
    choice2="SplashKit Manager"
    choice3="SplashKit dependencies"
    choice4="Visual Studio Code using snap Package"
    choice5=".Net 6 LTS Core SDK using apt Package"

    choices=$(
        zenity --list --checklist --column "Select" --column "Install" --width=400 --height=300 \
            TRUE "${choice1}" \
            TRUE "${choice2}" \
            TRUE "${choice3}" \
            TRUE "${choice4}" \
            TRUE "${choice5}")

    # Install CURL and GIT
    if [[ "${choices}" =~ "${choice1}" ]]; then
        sudo apt-get install curl git -y;
    fi

    # Install SplashKit Manager
    if [[ "${choices}" =~ "${choice2}" ]]; then
        bash <(curl -s https://raw.githubusercontent.com/splashkit/skm/master/install-scripts/skm-install.sh);
    fi

    # Install necessary dependencies and compile SplashKit
    if [[ "${choices}" =~ "${choice3}" ]]; then
        skm linux install -y;
    fi

    # Install Visual Studio Code using snap
    if [[ "${choices}" =~ "${choice4}" ]]; then
        sudo snap install code --classic;
    fi

    # Install the .NET 6 LTS Core SDK
    if [[ "${choices}" =~ "${choice5}" ]]; then
        sudo apt-get install -y dotnet6;
    fi

    zenity --info --text="All Installation  Complete"
}

# FOR macOS
# -----------------------------------------------------------
function MacInstall {
	read -p "Install xcode-select (y/n)? " installXcodeSelect
	read -p "Install SplashKit SDK (y/n)? " installSplashKitSDK
	read -p "Install Homebrew (y/n)? " installHomebrew
	read -p "Install .NET SDK using brew (y/n)? " installDotNet
	read -p "Install VS Code using brew (y/n)? " installVSCode

	# Install xcode-select
	if [[ "$installXcodeSelect" =~ ^([yY][eE][sS]|[yY]) ]]; then
		echo "Installing xcode-select ...";
		sudo rm -rf /Library/Developer/CommandLineTools;
		xcode-select --install;
	fi

	# Install SplashKit SDK
	if [[ "$installSplashKitSDK" =~ ^([yY][eE][sS]|[yY]) ]]; then
		echo "Installing SplashKit SDK ...";
		bash <(curl -s https://raw.githubusercontent.com/splashkit/skm/master/install-scripts/skm-install.sh);
	fi

	# Install Homebrew package manager
	if [[ "$installHomebrew" =~ ^([yY][eE][sS]|[yY]) ]]; then
		echo "Installing Homebrew ...";
		sudo yes | /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)";
	fi

	# Install .NET SDK using brew
	if [[ "$installDotNet" =~ ^([yY][eE][sS]|[yY]) ]]; then
		echo "Installing .NET SDK ...";
		brew install dotnet;
	fi

	# Install VS Code using brew
	if [[ "$installVSCode" =~ ^([yY][eE][sS]|[yY]) ]]; then
		echo "Installing VS Code ...";
		brew install --cask visual-studio-code;
	fi

    echo "Installation Complete!"
}

# Check if Linux or Darwin
uname=$(uname)
case "$uname" in
	(*Linux*) UbuntuInstall; ;;
	(*Darwin*) MacInstall; ;;
	(*) echo "ERROR"; ;;
esac;

exit #>

# FOR WINDOWS
# -----------------------------------------------------------
[reflection.assembly]::loadwithpartialname("System.Windows.Forms") | Out-Null
[reflection.assembly]::loadwithpartialname("System.Drawing") | Out-Null

$form = New-Object System.Windows.Forms.Form
$button = New-Object System.Windows.Forms.Button
$checkBox1 = New-Object System.Windows.Forms.CheckBox
$checkBox2 = New-Object System.Windows.Forms.CheckBox
$checkBox3 = New-Object System.Windows.Forms.CheckBox
$checkBox4 = New-Object System.Windows.Forms.CheckBox
$checkBox5 = New-Object System.Windows.Forms.CheckBox
$checkBox6 = New-Object System.Windows.Forms.CheckBox
$InitialFormWindowState = New-Object System.Windows.Forms.FormWindowState

$handler_button_Click= 
{
    # Install msys2
    if ($checkBox1.Checked) {
        $checkBox1.ForeColor = 'blue'
        $checkBox1.Text = "MSYS2 using Winget (Installing...)"
        & "$env:LOCALAPPDATA\Microsoft\WindowsApps\winget" install msys2
        $checkBox1.ForeColor = 'green'
        $checkBox1.Text = "MSYS2 using Winget (Done)"
    }
    Start-Sleep 1
    # Install VS Code
    if ($checkBox2.Checked) {
        $checkBox2.ForeColor = 'blue'
        $checkBox2.Text = "Visual Studio Code using Winget (Installing...)"
        & "$env:LOCALAPPDATA\Microsoft\WindowsApps\winget" install Microsoft.VisualStudioCode
        $checkBox2.ForeColor = 'green'
        $checkBox2.Text = "Visual Studio Code using Winget (Done)"
    }
    Start-Sleep 1
    # Install the .NET 6 LTS Core SDK on Windows for C#
    if ($checkBox3.Checked) {
        $checkBox3.ForeColor = 'blue'
        $checkBox3.Text = ".Net 6 LTS Core SDK using Winget (Installing...)"
        & "$env:LOCALAPPDATA\Microsoft\WindowsApps\winget" install Microsoft.DotNet.SDK.6
        $checkBox3.ForeColor = 'green'
        $checkBox3.Text = ".Net 6 LTS Core SDK using Winget (Done)"
    }
    Start-Sleep 1
    # Install Git
    if ($checkBox4.Checked) {
        $checkBox4.ForeColor = 'blue'
        $checkBox4.Text = "Git using pacman (Installing...)" 
        bash -l -c "pacman -S git --noconfirm --disable-download-timeout"
        $checkBox4.ForeColor = 'green'
        $checkBox4.Text = "Git using pacman (Done)"

    }
    Start-Sleep 1
    # Install SplashKit Manager
    if ($checkBox5.Checked) {
        $checkBox5.ForeColor = 'blue'
        $checkBox5.Text = "SplashKit SDK (Installing...)"
        bash -l -c "bash <(curl -s https://raw.githubusercontent.com/splashkit/skm/master/install-scripts/skm-install.sh)" 
        $checkBox5.ForeColor = 'green'
        $checkBox5.Text = "SplashKit SDK (Done)"
    }
    Start-Sleep 1
    # Installing GCC on Windows for C++
    if ($checkBox6.Checked) {
        $checkBox6.ForeColor = 'blue'
        $checkBox6.Text = "GCC for C++ (Installing...)"
        bash -l -c "pacman --noconfirm --disable-download-timeout -S mingw-w64-{x86_64,i686}-gcc mingw-w64-{i686,x86_64}-gdb"
        $checkBox6.ForeColor = 'green'
        $checkBox6.Text = "GCC for C++ (Done)"
    }
    Start-Sleep 1
    $form.Close() | Out-Null
}

$form.Text = "Select Tools to Install"
$form.Name = "form"
$form.Font = New-Object System.Drawing.Font("Arial",11)
$form.DataBindings.DefaultDataSourceUpdateMode = 0
$System_Drawing_Size = New-Object System.Drawing.Size
$System_Drawing_Size.Width = 500
$System_Drawing_Size.Height = 300
$form.ClientSize = $System_Drawing_Size

$button.TabIndex = 6
$button.Text = "Install"
$button.Name = "button"
$System_Drawing_Size = New-Object System.Drawing.Size
$System_Drawing_Size.Width = 80
$System_Drawing_Size.Height = 40
$button.Size = $System_Drawing_Size
$button.UseVisualStyleBackColor = $True

$System_Drawing_Point = New-Object System.Drawing.Point
$System_Drawing_Point.X = 25
$System_Drawing_Point.Y = 230
$button.Location = $System_Drawing_Point
$button.DataBindings.DefaultDataSourceUpdateMode = 0
$button.add_Click($handler_button_Click)

$form.Controls.Add($button)

$System_Drawing_Size = New-Object System.Drawing.Size
$System_Drawing_Size.Width = 500
$System_Drawing_Size.Height = 30

$checkBox1.UseVisualStyleBackColor = $True
$checkBox1.Size = $System_Drawing_Size
$checkBox1.TabIndex = 0
$checkBox1.Text = "Install MSYS2 using Winget"
$System_Drawing_Point = New-Object System.Drawing.Point
$System_Drawing_Point.X = 25
$System_Drawing_Point.Y = 15
$checkBox1.Location = $System_Drawing_Point
$checkBox1.DataBindings.DefaultDataSourceUpdateMode = 0
$checkBox1.Name = "checkBox1"
$checkBox1.Checked = $True

$form.Controls.Add($checkBox1)

$checkBox2.UseVisualStyleBackColor = $True
$checkBox2.Size = $System_Drawing_Size
$checkBox2.TabIndex = 1
$checkBox2.Text = "Install Visual Studio Code using Winget"
$System_Drawing_Point = New-Object System.Drawing.Point
$System_Drawing_Point.X = 25
$System_Drawing_Point.Y = 45
$checkBox2.Location = $System_Drawing_Point
$checkBox2.DataBindings.DefaultDataSourceUpdateMode = 0
$checkBox2.Name = "checkBox2"
$checkBox2.Checked = $True

$form.Controls.Add($checkBox2)

$checkBox3.UseVisualStyleBackColor = $True
$checkBox3.Size = $System_Drawing_Size
$checkBox3.TabIndex = 2
$checkBox3.Text = "Install .Net 6 LTS Core SDK using Winget"
$System_Drawing_Point = New-Object System.Drawing.Point
$System_Drawing_Point.X = 25
$System_Drawing_Point.Y = 75
$checkBox3.Location = $System_Drawing_Point
$checkBox3.DataBindings.DefaultDataSourceUpdateMode = 0
$checkBox3.Name = "checkBox3"
$checkBox3.Checked = $True

$form.Controls.Add($checkBox3)

$checkBox4.UseVisualStyleBackColor = $True
$checkBox4.Size = $System_Drawing_Size
$checkBox4.TabIndex = 3
$checkBox4.Text = "Install Git using pacman"
$System_Drawing_Point = New-Object System.Drawing.Point
$System_Drawing_Point.X = 25
$System_Drawing_Point.Y = 105
$checkBox4.Location = $System_Drawing_Point
$checkBox4.DataBindings.DefaultDataSourceUpdateMode = 0
$checkBox4.Name = "checkBox4"
$checkBox4.Checked = $True

$form.Controls.Add($checkBox4)

$checkBox5.UseVisualStyleBackColor = $True
$checkBox5.Size = $System_Drawing_Size
$checkBox5.TabIndex = 4
$checkBox5.Text = "Install SplashKit SDK"
$System_Drawing_Point = New-Object System.Drawing.Point
$System_Drawing_Point.X = 25
$System_Drawing_Point.Y = 135
$checkBox5.Location = $System_Drawing_Point
$checkBox5.DataBindings.DefaultDataSourceUpdateMode = 0
$checkBox5.Name = "checkBox5"
$checkBox5.Checked = $True

$form.Controls.Add($checkBox5)

$checkBox6.UseVisualStyleBackColor = $True
$checkBox6.Size = $System_Drawing_Size
$checkBox6.TabIndex = 5
$checkBox6.Text = " Install GCC for C++"
$System_Drawing_Point = New-Object System.Drawing.Point
$System_Drawing_Point.X = 25
$System_Drawing_Point.Y = 165
$checkBox6.Location = $System_Drawing_Point
$checkBox6.DataBindings.DefaultDataSourceUpdateMode = 0
$checkBox6.Name = "checkBox6"
$checkBox6.Checked = $True

$form.Controls.Add($checkBox6)

$InitialFormWindowState = $form.WindowState
$form.ShowDialog() | Out-Null
