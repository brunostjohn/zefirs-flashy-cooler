window.addEventListener("load", (event) => {
    window.electronAPI.getThemeList();
    window.electronAPI.requestConfig();
    window.electronAPI.renderStatus();
    window.electronAPI.requestHealth();
})

const body = document.getElementById("card-container");

window.electronAPI.receiveThemeList((_event, theme) => {
    let htmlToAppend = "<div class='card' onclick='themeSelect(\"" + theme.id + "\");'><img src='" + theme.preview + "' class='card-img' />" + "<h5 class='card-title'>" + theme.title;
        htmlToAppend += "<span class='active-badge badge text-bg-success rounded-pill' id='" + theme.id + "'>Active</span>"
        htmlToAppend += "</h5><p class='card-text'>" + theme.description + "</p></div>";
    body.insertAdjacentHTML("beforeend", htmlToAppend);
    if(theme.isActive) {
        document.getElementById(theme.id).style.opacity = "1";
    } else {
        document.getElementById(theme.id).style.opacity = "0";
    }
});

window.electronAPI.updateRenderStatus((_event, value) => {
    const elements = document.getElementsByClassName("active-badge");
    if (value == 0){
        for (let i = 0; i<elements.length; i++) {
            elements[i].classList.remove("text-bg-success");
            elements[i].classList.add("text-bg-warning");
            elements[i].innerHTML = "Inactive";
        }
    } else if (value==1) {
        elements[i].classList.remove("text-bg-warning");   
        elements[i].classList.add("text-bg-success");
        elements[i].innerHTML = "Active";
    }
});

function themeSelect(themeId) {
    window.electronAPI.themeSelected(themeId);
    const elements = document.getElementsByClassName("active-badge");
    for (let i = 0; i<elements.length; i++) {
        elements[i].style.opacity = "0";
    }
    document.getElementById(themeId).style.opacity = "1";
}

const alertPlaceholder = document.getElementById("liveAlertPlaceholder");

const alert1 = (message, type) => {
    const wrapper = document.createElement("div");
    wrapper.innerHTML = [
      `<div class="alert alert-${type} alert-dismissible" role="alert">`,
      `   <div>${message}</div>`,
      '   <button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>',
      '</div>'
    ].join('')
  
    alertPlaceholder.append(wrapper);
}

let showWarningAlert;

window.electronAPI.receiveConfig((_event, value) => {
    showWarningAlert = value.showWarningAlert;
});

window.electronAPI.receiveHealth((_event, value) => {
    if(value[0]) {
        alert1("iCUE is running. DO NOT start rendering. Please check the Settings' Health segment!", "danger");
    }
    if (!value[1]) {
        if(showWarningAlert) { alert1("LibreHardwareMonitor isn't running. Please check the Settings' Health segment! If you don't wish to receive this alert, please disable it in the Settings.", "warning"); }
    }
});