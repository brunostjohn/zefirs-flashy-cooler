window.addEventListener("load", (event) => {
  window.electronAPI.getThemeList();
  window.electronAPI.requestConfig();
  window.electronAPI.renderStatus();
  window.electronAPI.requestHealth();
});

const body = document.getElementById("card-container");

window.electronAPI.threeDivs((_event) => {
  // <div id="finalCard">
  //   <a href="themedownload.html" id="linkingContainer">
  //     <img src="../images/circle-down-regular.svg" id="downloadIcon" />
  //     <p id="downloadText">Download more themes.</p>
  //   </a>
  // </div>
  let htmlToAppend = `
    <div class="fillspace"></div>
    <div class="fillspace"></div>
    <div class="fillspace"></div>
    `;
  body.insertAdjacentHTML("beforeend", htmlToAppend);
});

window.electronAPI.receiveThemeList((_event, theme) => {
  let htmlToAppend =
    "<div class='theme-card' onclick='themeSelect(\"" +
    theme.id +
    "\");'><img src='" +
    theme.preview +
    "' class='theme-img' />" +
    "<h5 class='theme-title'>" +
    theme.title;
  htmlToAppend +=
    "</h5><img class='active-badge' id='" +
    theme.id +
    "' src='../images/circle-check-solid.svg' />";
  htmlToAppend += "</div>";
  body.insertAdjacentHTML("beforeend", htmlToAppend);
  if (theme.isActive) {
    document.getElementById(theme.id).style.visibility = "visible";
  } else {
    document.getElementById(theme.id).style.visibility = "hidden";
  }
});

window.electronAPI.updateRenderStatus((_event, value) => {
  const elements = document.getElementsByClassName("active-badge");
  if (value == 0) {
    for (let i = 0; i < elements.length; i++) {
      elements[i].classList.remove("active");
      elements[i].classList.add("inactive");
    }
  } else if (value == 1) {
    elements[i].classList.remove("inactive");
    elements[i].classList.add("active");
  }
});

function themeSelect(themeId) {
  window.electronAPI.themeSelected(themeId);
  const elements = document.getElementsByClassName("active-badge");
  for (let i = 0; i < elements.length; i++) {
    elements[i].style.visibility = "hidden";
  }
  document.getElementById(themeId).style.visibility = "visible";
}

const alertPlaceholder = document.getElementById("liveAlertPlaceholder");

const alert1 = (message, type) => {
  const wrapper = document.createElement("div");
  wrapper.innerHTML = [
    `<div class="alert alert-${type} alert-dismissible" role="alert">`,
    `   <div>${message}</div>`,
    '   <button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>',
    "</div>",
  ].join("");

  alertPlaceholder.append(wrapper);
};

let showWarningAlert;

window.electronAPI.receiveConfig((_event, value) => {
  showWarningAlert = value.showWarningAlert;
});

window.electronAPI.receiveHealth((_event, value) => {
  if (value[0]) {
    alert1(
      "iCUE is running. DO NOT start rendering. Please check the Settings' Health segment!",
      "danger"
    );
  }
  if (!value[1]) {
    if (showWarningAlert) {
      alert1(
        "LibreHardwareMonitor isn't running. Please check the Settings' Health segment! If you don't wish to receive this alert, please disable it in the Settings.",
        "warning"
      );
    }
  }
});
