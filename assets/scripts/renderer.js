const btn = document.getElementById("btn");

window.addEventListener("load", (event) => {
    Coloris({
        themeMode: 'dark',
        theme: "pill",
        alpha: false,
        swatches: [
            '#264653',
            '#2a9d8f',
            '#e9c46a',
            'rgb(244,162,97)',
            '#e76f51',
            '#d62828',
            'navy',
            '#07b',
            '#0096c7',
            '#00b4d880',
            'rgb(0,119,182)'
          ],
      });
    window.electronAPI.renderStatus();
    window.electronAPI.getThemeList();
})

function makeId(length) {
    var result = "";
    var characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
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

const reset = document.getElementById("reset");

reset.addEventListener("click", () => {
    parameters.forEach(controllableParameter => {
        const controllingElement = document.getElementById(controllableParameter.id);
        controllableParameter.value = controllableParameter.defaultValue;
        controllingElement.value = controllableParameter.defaultValue;
    });
    window.electronAPI.parametersSendback(parameters);

    window.location.reload();
});

function createControllableParameter(controllableParameter) {
    const form = document.getElementById("parameterContainer");
    let htmlToAppend = "";
    if(controllableParameter.type == "colour") {
        htmlToAppend += "<h4>" + controllableParameter.title + "</h4><input type='text' class='reset-this form-control' id='" + controllableParameter.id + "' value='" + controllableParameter.value + "' data-coloris /><span class='badge rounded-pill' id='" + controllableParameter.id + "badge'" + ">Current colour</span>";
        form.insertAdjacentHTML("beforeend", htmlToAppend);
        const colour = document.getElementById(controllableParameter.id);
        const colourBadge = document.getElementById(controllableParameter.id + "badge");
        colourBadge.style.backgroundColor = colour.value;
        colour.addEventListener("input", () => {
            colourBadge.style.backgroundColor = colour.value;
        });
        htmlToAppend = "";
    } else if (controllableParameter.type == "file") {
        htmlToAppend += "<h4>" + controllableParameter.title + "</h4><button type='button' class='btn btn-outline-info' id='" + controllableParameter.id + "'>Open File</button>";
        form.insertAdjacentHTML("beforeend", htmlToAppend);
        const button = document.getElementById(controllableParameter.id);
        button.addEventListener("click", async () => {
            const filePath = await window.electronAPI.openFile();
            button.value = filePath;
        });
        htmlToAppend = "";
    } else if (controllableParameter.type == "range") {
        htmlToAppend += "<h4>" + controllableParameter.title + "</h4><input type='range' class='form-range' min='" + controllableParameter.min + "' max='" + controllableParameter.max + "' step='" + controllableParameter.step + "' id='" + controllableParameter.id + "' value='" + controllableParameter.value + "' /><label for='" + controllableParameter.id + "' class='form-label' id='" + controllableParameter.id + "label'>Current value: " + controllableParameter.value +"</label>";
        form.insertAdjacentHTML("beforeend", htmlToAppend);
        const range = document.getElementById(controllableParameter.id);
        const rangeLabel = document.getElementById(controllableParameter.id + "label");
        range.addEventListener("input", () => {
            rangeLabel.textContent = "Current value: " + range.value;
        });
        htmlToAppend = "";
    } else if (controllableParameter.type == "sensor") {
        appendSensors(controllableParameter);
    }
    htmlToAppend += "<br /><br />";
    parameters.push(controllableParameter);
    form.insertAdjacentHTML("beforeend", htmlToAppend);
}

function appendSensors(controllableParameter) {
    window.electronAPI.requestSensorInfoForPath([controllableParameter.value, controllableParameter.id]);
}