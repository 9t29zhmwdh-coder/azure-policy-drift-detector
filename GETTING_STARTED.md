# Getting Started (Beginner Guide)

This guide walks you through running **Azure Policy Drift Detector** (`apdd`) from scratch, even if you have never used a terminal or Rust before. No prior knowledge required.

The tool is a command-line program only, no graphical interface, no installer. You type commands into a terminal and read the output there.

---

## Windows

### 1. Open a terminal

Right-click the **Start** button and choose **Terminal** (or **PowerShell** on older Windows versions).

### 2. Check if Rust is installed

Type the following commands and press Enter after each:

```powershell
rustc --version
cargo --version
```

If you see version numbers (e.g. `rustc 1.80.0`), Rust is installed, skip to step 3.

If you instead see something like `rustc is not recognized as an internal or external command`, Rust is either not installed or not in your system's PATH.

Install it from **[https://rustup.rs](https://rustup.rs)**:

1. Download `rustup-init.exe` from the page.
2. Run it and follow the prompts (the default options are fine).
3. Close and reopen your terminal after installation so it picks up the new PATH.
4. Run `rustc --version` again to confirm it works.

### 3. Get the code

You don't need to know Git for this.

1. Go to the repository page: [https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector](https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector)
2. Click the green **Code** button.
3. Click **Download ZIP**.
4. Extract the ZIP file to a folder of your choice (e.g. `C:\apdd`).

<!-- TODO: Screenshot of the green Code button and Download ZIP option -->

**Alternative (if you have Git installed):**

```powershell
git clone https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector.git
```

### 4. Build the project

Open your terminal in the extracted (or cloned) folder, then run:

```powershell
cargo build --release
```

This downloads dependencies and compiles the tool. It can take a few minutes the first time.

### 5. Run it

Try the built-in demo first, it needs no Azure credentials at all:

```powershell
.\target\release\apdd.exe demo
```

This runs the tool against a built-in synthetic (fake) subscription so you can see exactly what output to expect before connecting to a real Azure environment.

Once you're ready to scan a real subscription, follow the **Requirements** and **App Registration Setup** sections in the main [README.md](README.md), then run:

```powershell
.\target\release\apdd.exe scan
```

### What happens after running

The tool prints a table to your terminal listing scanned resources, non-compliant resources, and drift findings, each with a severity (Critical, High, Medium, Low, Informational), the affected resource, and the policy that was violated. See the "Sample Output" section in the [README](README.md) for a full example.

---

## Linux

### 1. Open a terminal

This depends on your desktop environment. Look for an app named **Terminal**, **Konsole**, or similar in your application menu, most desktops let you search for "Terminal" from the app launcher.

### 2. Check if Rust is installed

```bash
rustc --version
cargo --version
```

If you see version numbers, Rust is installed, skip to step 3.

If you see `command not found: rustc`, Rust is not installed or not in your PATH.

Install it from **[https://rustup.rs](https://rustup.rs)** using the curl one-liner shown on that page:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen prompts (default options are fine), then close and reopen your terminal so the PATH updates take effect. Confirm with `rustc --version`.

### 3. Get the code

You don't need to know Git for this.

1. Go to the repository page: [https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector](https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector)
2. Click the green **Code** button.
3. Click **Download ZIP**.
4. Extract the ZIP file to a folder of your choice.

**Alternative (if you have Git installed):**

```bash
git clone https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector.git
```

### 4. Build the project

Open a terminal in the extracted (or cloned) folder, then run:

```bash
cargo build --release
```

### 5. Run it

Try the built-in demo first, no Azure credentials needed:

```bash
./target/release/apdd demo
```

Once ready for a real subscription, follow the **Requirements** and **App Registration Setup** sections in [README.md](README.md), then run:

```bash
./target/release/apdd scan
```

### What happens after running

You'll see a table in your terminal with scanned resources, non-compliant resources, and drift findings by severity, plus which policy was violated for each finding. See the "Sample Output" section in the [README](README.md).

---

## macOS

### 1. Open a terminal

Press **Cmd+Space** to open Spotlight, type `Terminal`, and press Enter.

### 2. Check if Rust is installed

```bash
rustc --version
cargo --version
```

If you see version numbers, Rust is installed, skip to step 3.

If you see `command not found: rustc`, Rust is not installed or not in your PATH.

Install it from **[https://rustup.rs](https://rustup.rs)** using the curl one-liner shown on that page:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts, then close and reopen your terminal. Confirm with `rustc --version`.

### 3. Get the code

You don't need to know Git for this.

1. Go to the repository page: [https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector](https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector)
2. Click the green **Code** button.
3. Click **Download ZIP**.
4. Extract the ZIP file to a folder of your choice.

**Alternative (if you have Git installed):**

```bash
git clone https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector.git
```

### 4. Build the project

```bash
cargo build --release
```

### 5. Run it

Try the built-in demo first, no Azure credentials needed:

```bash
./target/release/apdd demo
```

Once ready for a real subscription, follow the **Requirements** and **App Registration Setup** sections in [README.md](README.md), then run:

```bash
./target/release/apdd scan
```

### What happens after running

You'll see a table in your terminal with scanned resources, non-compliant resources, and drift findings by severity, plus which policy was violated for each finding. See the "Sample Output" section in the [README](README.md).

---

### Troubleshooting

| Problem | Cause | Fix |
|---|---|---|
| `rustc`/`cargo` still "not recognized" / "command not found" after installing Rust | Terminal session still has the old PATH loaded | Close the terminal window completely and open a new one, then try again |
| `cargo build --release` fails on Windows with linker errors (e.g. `link.exe not found`) | Missing C++ Build Tools required by the Rust toolchain on Windows | Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the "Desktop development with C++" workload, then retry the build |
| `apdd scan` hangs or fails to reach Azure (timeouts, connection errors) | Network, proxy, or firewall blocking access to Azure management endpoints | Ensure outbound access to `login.microsoftonline.com` and `management.azure.com` is allowed; if behind a corporate proxy, configure your proxy environment variables (e.g. `HTTPS_PROXY`) before running the tool |
| `apdd demo` works but `apdd scan` fails with an authentication error | Missing or incorrect credentials in `.env` | Double-check `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`, and `AZURE_SUBSCRIPTION_ID` in your `.env` file against the values from your App Registration (see README) |
