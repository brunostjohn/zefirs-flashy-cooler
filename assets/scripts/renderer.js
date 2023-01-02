const btn = document.getElementById("btn");

window.addEventListener("load", (event) => {
    window.electronAPI.renderStatus();
    window.electronAPI.getThemeList();
})

function makeId(length) {
    var result           = '';
    var characters       = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    var charactersLength = characters.length;
    for ( var i = 0; i < length; i++ ) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
    }
    return result;
}
    
window.electronAPI.updateRenderStatus((_event, value) => {
    if (value == 1){
        btn.textContent = "Stop Rendering";
        btn.classList.remove("btn-success");
        btn.classList.add("btn-danger");
    } else if (value==0) {
        btn.textContent = "Start Rendering";
        btn.classList.remove("btn-danger");
        btn.classList.add("btn-success");    
    }
});

btn.addEventListener("click", () => {
    if (btn.textContent.toLowerCase().includes("Start Rendering".toLowerCase())) {
        window.electronAPI.startRendering()
        btn.textContent = "Stop Rendering";
      } else {
        window.electronAPI.stopRendering()
        btn.textContent = "Start Rendering";
      }
})

window.electronAPI.receiveThemeList((_event, theme) => {
    if(theme.isActive) {
        document.getElementById("preview").src=theme.preview;
        document.getElementById("title").innerText=theme.title;
        document.getElementById("themeName").innerText=theme.title;
        document.getElementById("themeDescription").innerText=theme.description;
        if(theme.hasConfig){
            const controllableParameters = theme.controllableParameters;
            Object.keys(controllableParameters).forEach(key => {
                createControllableParameter(controllableParameters[key]);
            });
        } else {
            const container = document.getElementById("parameters");
            container.style.visibility = "hidden";
        }
    }
});

const apply = document.getElementById("apply");

let parameters = [];

apply.addEventListener("click", () => {
    parameters.forEach(controllableParameter => {
        const controllingElement = document.getElementById(controllableParameter.id);
        controllableParameter.value = controllingElement.value;
    });
    window.electronAPI.parametersSendback(parameters);

    window.location.reload();
});

// TODO: Add default values back!

function createControllableParameter(controllableParameter) {
    const form = document.getElementById("parameterContainer");
    let htmlToAppend = "";
    if(controllableParameter.type == "colour") {
        htmlToAppend += "<input type='color' id='" + controllableParameter.id + "' value='" + controllableParameter.value +"' /><label for='" + controllableParameter.id + "' class='form-label'>" + controllableParameter.title + "</label>";
    } else if (controllableParameter.type == "file") {
        htmlToAppend += "<label for='>" + controllableParameter.id + "'class='form-label'>" + controllableParameter.title + "</label><button type='button' class='btn btn-outline-info' id='" + controllableParameter.id + "'>Open File</button>";
        form.insertAdjacentHTML("beforeend", htmlToAppend);
        const button = document.getElementById(controllableParameter.id);
        button.addEventListener("click", async () => {
            const filePath = await window.electronAPI.openFile();
            button.value = filePath;
        });
        htmlToAppend = "";
    }
    htmlToAppend += "<br /><br />";
    parameters.push(controllableParameter);
    form.insertAdjacentHTML("beforeend", htmlToAppend);
}