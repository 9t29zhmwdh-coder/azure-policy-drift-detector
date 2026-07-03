<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>Azure Policy Drift Detector</h1>
</div>

> 🇬🇧 [English Version](README.md)

**Read-only Rust CLI zur Erkennung von Azure Policy Drift in Subscriptions, Priorisierung nicht-konformer Ressourcen und Generierung von Massnahmenberichten.**

Der Azure Policy Drift Detector verbindet sich per Anwendungsberechtigungen mit Azure Resource Graph und Policy Insights und vergleicht Ressourcenkonfigurationen mit aktiven Policy-Zuweisungen. Vollständig read-only, keine Daten verlassen das lokale Gerät.

Ausgerichtet am [Microsoft Cloud Security Benchmark (MCSB)](https://learn.microsoft.com/de-de/security/benchmark/azure/overview) und konzipiert für Azure Governance und Compliance-Teams.

[![CI](https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector/actions) ![Azure Ready](https://img.shields.io/badge/Azure-Ready-0078d4?logo=microsoftazure&logoColor=white) ![Platform](https://img.shields.io/badge/Platform-Windows_%7C_Ubuntu-lightgrey) ![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white) ![AI | Claude Code](https://img.shields.io/badge/AI-Claude_Code-black?logo=anthropic&logoColor=white) ![AI | Copilot](https://img.shields.io/badge/AI-Copilot-black?logo=github&logoColor=white) [![Release](https://img.shields.io/github/v/release/9t29zhmwdh-coder/azure-policy-drift-detector?color=3F8E7E)](https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector/releases) [![License](https://img.shields.io/github/license/9t29zhmwdh-coder/azure-policy-drift-detector?color=lightgrey)](LICENSE)

---

## Funktionen

| Funktion | Beschreibung |
|---|---|
| Ressourcenerkennung | Abfrage aller Ressourcen in einer Subscription via Azure Resource Graph (KQL) |
| Policy-Zustandsabruf | Aktuelle Compliance-Zustände aus Azure Policy Insights |
| Drift-Erkennung | Erkennt nicht-konforme Konfigurationen, Tag-Abweichungen und Policy-Ausnahmen |
| Risikopriorisierung | Klassifiziert Befunde nach Schweregrad basierend auf Policy-Kategorie |
| JSON-Export | Maschinenlesbarer Compliance-Bericht für Ticketing-Systeme |
| Markdown-Export | Lesbarer Bericht mit Befunden, Beschreibungen und Massnahmen |
| SARIF-Stub | Vorbereitet für GitHub Advanced Security Integration (v0.2) |
| GitHub Action Vorlage | Einsatzbereite Workflow-Vorlage für geplante Compliance-Prüfungen |

---

## Benötigte Azure RBAC Rollen

Registriere eine Anwendung in Entra ID und weise folgende Rollen auf Subscription-Ebene zu:

| Rolle | Zweck |
|---|---|
| `Reader` | Azure Resource Graph Abfragen |
| `Policy Insights Data Reader` | Policy Compliance-Zustände lesen |

Beide Rollen sind read-only. Schreibberechtigungen werden nicht benötigt und nicht verwendet.

---

## App-Registrierung einrichten

1. Im [Azure Portal](https://portal.azure.com) zu **Entra ID > App-Registrierungen > Neue Registrierung** navigieren
2. Anwendung benennen (z.B. `apdd-scanner`) und registrieren
3. Zur **Subscription > Zugriffssteuerung (IAM) > Rollenzuweisung hinzufügen** navigieren
4. `Reader` und `Policy Insights Data Reader` der Anwendung zuweisen
5. Unter **Entra ID > App-Registrierungen > App > Zertifikate und Geheimnisse > Neuer geheimer Clientschlüssel** den Wert kopieren
6. **Mandanten-ID**, **Client-ID**, **Geheimer Clientschlüssel** und **Subscription-ID** notieren

---

## Schnellstart

```bash
git clone https://github.com/9t29zhmwdh-coder/azure-policy-drift-detector
cd azure-policy-drift-detector

cp .env.example .env
# Zugangsdaten eintragen

cargo build --release

# Drift-Scan durchführen
./target/release/apdd scan

# Nur kritische und hohe Schweregrade anzeigen
./target/release/apdd scan --min-severity high

# Export als Markdown-Bericht
./target/release/apdd export --format md --output bericht.md

# Export als JSON
./target/release/apdd export --format json --output bericht.json
```

---

## Konfiguration

```env
AZURE_TENANT_ID=deine-mandanten-id
AZURE_CLIENT_ID=deine-client-id
AZURE_CLIENT_SECRET=dein-clientschluessel
AZURE_SUBSCRIPTION_ID=deine-subscription-id
```

Die `.env`-Datei ist in `.gitignore` aufgeführt. Zugangsdaten werden nie committet.

---

## Drift-Schweregrade

| Stufe | Auslöser | Beispiele |
|---|---|---|
| Critical | Sicherheitsrelevante Policy-Verletzung | Netzwerksicherheit, Verschlüsselung, Identität |
| High | Compliance-Framework-Verletzung | ISO, NIST, CIS-Benchmark-Policies |
| Medium | Tag-Abweichung oder fehlendes Pflicht-Tag | Kostenstellen-Tag, Umgebungs-Tag |
| Low | Geringfügige Konfigurationsabweichung | Namenskonvention |
| Informational | Aktive Policy-Ausnahme | Ressourcen mit gültiger Ausnahme |

---

## Voraussetzungen

- Rust 1.78+
- Azure Subscription mit App-Registrierung
- Netzwerkzugang zu `login.microsoftonline.com` und `management.azure.com`

---

**Autor:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Active · ![version](https://img.shields.io/github/v/release/9t29zhmwdh-coder/azure-policy-drift-detector?color=6b7280&style=flat-square) · **Lizenz:** MIT
