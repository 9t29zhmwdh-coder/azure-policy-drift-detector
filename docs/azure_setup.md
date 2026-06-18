# Azure Setup Guide

## Step 1: Register an application in Entra ID

1. Open the [Azure Portal](https://portal.azure.com)
2. Navigate to **Entra ID > App registrations > New registration**
3. Set a name (e.g. `apdd-scanner`)
4. Select **Accounts in this organizational directory only**
5. Leave the Redirect URI empty
6. Click **Register**

## Step 2: Assign RBAC roles at subscription scope

1. Navigate to your **Subscription > Access control (IAM)**
2. Click **Add role assignment**
3. Assign the `Reader` role to your application
4. Repeat for the `Policy Insights Data Reader` role

Both roles are read-only. The tool does not require any write permissions.

## Step 3: Create a client secret

1. Go to **Entra ID > App registrations > your app > Certificates and secrets**
2. Click **New client secret**
3. Set an expiry (6 months recommended for security tooling)
4. Copy the **Value** immediately

## Step 4: Configure credentials

Create a `.env` file in the project root:

```env
AZURE_TENANT_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
AZURE_CLIENT_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
AZURE_CLIENT_SECRET=your-secret-value
AZURE_SUBSCRIPTION_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
```

Find these values on the app registration overview page:

- **Tenant ID:** shown as "Directory (tenant) ID"
- **Client ID:** shown as "Application (client) ID"
- **Subscription ID:** shown on the Subscription overview page
