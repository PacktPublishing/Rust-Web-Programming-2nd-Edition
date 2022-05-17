const { app, BrowserWindow } = require("electron");
const path = require("path");
const isDev = require("electron-is-dev");


function createWindow() {
    const mainWindow = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            nodeIntegration: true,
            enableRemoteModule: true,
            contextIsolation: false,
        },
    });
    mainWindow.loadURL(
        isDev
           ? "http://localhost:3000"
           : `file://${path.join(__dirname, "../build/index.html")}`
    );
    if (isDev) {
        mainWindow.webContents.openDevTools();
    }
}


app.whenReady().then(() => {
    createWindow();
    app.on("activate", function () {
        if (BrowserWindow.getAllWindows().length === 0){
           createWindow(); 
        }
    });
});


app.on("window-all-closed", function () {
    if (process.platform !== "darwin") app.quit();
});
