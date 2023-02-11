window.addEventListener("load", (event) => {
    window.electronAPI.requestConfig();
    window.electronAPI.requestVersion();
    window.electronAPI.requestHealth();
    window.electronAPI.requestThemeFolder();
});

const version = document.getElementById("version");

window.electronAPI.receiveVersion((_event, value) => {
    version.textContent = "Version: " + value;
});

const startAtLogin = document.getElementById("loginSwitch");
const renderAtStartup = document.getElementById("renderAtStartupSwitch");
const startMinimised = document.getElementById("startMinimisedSwitch");
const warningAlert = document.getElementById("warningAlertSwitch");

let config;

window.electronAPI.receiveConfig((_event, value) => {
    config = value;
    startAtLogin.checked = config.startAtLogin;
    renderAtStartup.checked = config.renderAtStartup;
    startMinimised.checked = config.startMinimised;
    warningAlert.checked = config.showWarningAlert;
});

const applyButton = document.getElementById("apply");

applyButton.addEventListener("click", () => {
    config.startAtLogin = startAtLogin.checked;
    config.renderAtStartup = renderAtStartup.checked;
    config.startMinimised = startMinimised.checked;
    config.showWarningAlert = warningAlert.checked;
    window.electronAPI.configSendback(config);
})

const icue = document.getElementById("icue");
const libre = document.getElementById("libre");

window.electronAPI.receiveHealth((_event, value) => {
    icue.classList.add(value[0] ? "text-bg-danger" : "text-bg-success");
    icue.textContent = value[0] ? "Running" : "Not Running";
    libre.classList.add(value[1] ? "text-bg-success" : "text-bg-warning");
    libre.textContent = value[1] ? "Running" : "Not Running";
});

const themeFolderText = document.getElementById("themeFolderText");

window.electronAPI.receiveThemeFolder((_event, value) => {
    themeFolderText.textContent = "Current theme folder: " + value;
});

const themeFolderBtn = document.getElementById("themeFolderBtn");

themeFolderBtn.addEventListener("click", () => {
    window.electronAPI.openThemeFolder();
});