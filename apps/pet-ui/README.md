# Pet UI

Eine React-basierte Benutzeroberfläche zur Verwaltung von Haustieren über die Pet Service REST API.

## Features

- 📋 **Liste aller Haustiere** anzeigen
- ➕ **Neue Haustiere anlegen** mit Name, Art und Alter
- ✏️ **Haustiere bearbeiten** durch Klick auf "Edit"
- 🗑️ **Haustiere löschen** mit Bestätigungsdialog
- 🎨 **Responsive Design** für Desktop und Mobile
- ⚡ **Live-Updates** nach jeder Operation

## Technologie-Stack

- **React 19** mit TypeScript
- **Vite** als Build-Tool und Dev-Server
- **SCSS Modules** für komponentenbasiertes Styling
- **Nx** für Monorepo-Management

## API-Integration

Die UI kommuniziert mit dem `web_service` Backend über folgende Endpunkte:

- `GET /pets` - Alle Haustiere abrufen
- `GET /pets/{id}` - Ein spezifisches Haustier abrufen
- `POST /pets` - Neues Haustier anlegen
- `PUT /pets/{id}` - Haustier aktualisieren
- `DELETE /pets/{id}` - Haustier löschen

## Entwicklung

### Voraussetzungen

1. Backend-Service muss laufen:

   ```bash
   npx nx run web_service
   ```

2. UI-Dev-Server starten:
   ```bash
   npx nx serve pet-ui
   ```

Die UI ist dann unter `http://localhost:4200` erreichbar.

### Proxy-Konfiguration

Der Vite-Dev-Server ist so konfiguriert, dass API-Anfragen an `/api/*` automatisch an `http://localhost:8080` weitergeleitet werden. Dies vermeidet CORS-Probleme während der Entwicklung.

### CORS

Das Backend ist so konfiguriert, dass es CORS-Anfragen von `http://localhost:4200` akzeptiert.

## Testen

```bash
# Unit Tests
npx nx test pet-ui

# E2E Tests (falls konfiguriert)
npx nx e2e pet-ui-e2e
```

## Build

```bash
# Production Build
npx nx build pet-ui

# Build-Output ist in dist/apps/pet-ui
```

## Projekt-Struktur

```
apps/pet-ui/
├── src/
│   ├── app/
│   │   ├── app.tsx           # Hauptkomponente mit Pet-Management-Logik
│   │   ├── app.module.scss   # Styles für die App
│   │   └── app.spec.tsx      # Tests
│   ├── main.tsx               # App-Entry-Point
│   └── styles.scss            # Globale Styles
├── vite.config.ts             # Vite-Konfiguration mit Proxy
└── project.json               # Nx-Projektkonfiguration
```

## Verwendung

1. **Haustier anlegen**: Fülle das Formular links aus (Name, Art, Alter) und klicke auf "Create Pet"
2. **Haustier bearbeiten**: Klicke auf "Edit" bei einem Haustier, ändere die Daten und klicke auf "Update Pet"
3. **Haustier löschen**: Klicke auf "Delete" und bestätige die Löschung
4. **Bearbeitung abbrechen**: Klicke auf "Cancel" um zum Erstellungs-Modus zurückzukehren

## Troubleshooting

**Problem**: "Failed to fetch pets"

- **Lösung**: Stelle sicher, dass der Backend-Service unter `http://localhost:8080` läuft

**Problem**: CORS-Fehler

- **Lösung**: Backend wurde mit `actix-cors` konfiguriert, um Anfragen von localhost:4200 zu akzeptieren

**Problem**: Port 4200 bereits belegt

- **Lösung**: Ändere den Port in `vite.config.ts` oder beende den anderen Prozess
