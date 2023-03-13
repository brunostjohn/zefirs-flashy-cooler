const btn = document.getElementById("btn");

window.addEventListener("load", (event) => {
  Coloris({
    themeMode: "dark",
    theme: "pill",
    alpha: false,
    swatches: [
      "#264653",
      "#2a9d8f",
      "#e9c46a",
      "rgb(244,162,97)",
      "#e76f51",
      "#d62828",
      "navy",
      "#07b",
      "#0096c7",
      "#00b4d880",
      "rgb(0,119,182)",
    ],
  });
  window.electronAPI.requestSensorInfo();
  window.electronAPI.renderStatus();
  window.electronAPI.getThemeList();
});

function makeId(length) {
  var result = "";
  var characters =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  var charactersLength = characters.length;
  for (var i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
}

let sensorinfo;

window.electronAPI.receiveSensorInfo((_event, value) => {
  sensorinfo = value;
});

window.electronAPI.updateRenderStatus((_event, value) => {
  if (value == 1) {
    btn.textContent = "Stop Rendering";
    btn.classList.remove("btn-success");
    btn.classList.add("btn-danger");
  } else if (value == 0) {
    btn.textContent = "Start Rendering";
    btn.classList.remove("btn-danger");
    btn.classList.add("btn-success");
  }
});

btn.addEventListener("click", () => {
  if (btn.textContent.toLowerCase().includes("Start Rendering".toLowerCase())) {
    window.electronAPI.startRendering();
    btn.textContent = "Stop Rendering";
  } else {
    window.electronAPI.stopRendering();
    btn.textContent = "Start Rendering";
  }
});

window.electronAPI.receiveThemeList((_event, theme) => {
  if (theme.isActive) {
    document.getElementById("preview").src = theme.preview;
    document.getElementById("title").innerText = theme.title;
    document.getElementById("themeName").innerText = theme.title;
    document.getElementById("themeDescription").innerText = theme.description;
    if (theme.hasConfig) {
      const controllableParameters = theme.controllableParameters;
      Object.keys(controllableParameters).forEach((key) => {
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

let lasttime = Date.now();

apply.addEventListener("click", () => {
  if (Date.now() - 2000 > lasttime) {
    // fuck you chromium
    parameters.forEach((controllableParameter) => {
      const controllingElement = document.getElementById(
        controllableParameter.id
      );
      if (controllableParameter.type == "bool") {
        controllableParameter.value = controllingElement.checked;
      } else {
        controllableParameter.value = controllingElement.value;
      }
    });
    window.electronAPI.parametersSendback(parameters);

    window.location.reload();
  }
});

window.electronAPI.receiveFreshPreview((_event, value) => {
  document.getElementById("preview").src = value;
});

const reset = document.getElementById("reset");

let lasttimevol2 = Date.now();

reset.addEventListener("click", () => {
  if (Date.now() - 2000 > lasttimevol2) {
    parameters.forEach((controllableParameter) => {
      if (controllableParameter.type != "sensor") {
        const controllingElement = document.getElementById(
          controllableParameter.id
        );
        if (controllableParameter.type == "bool") {
          controllableParameter.value = controllableParameter.defaultValue;
          controllingElement.checked = controllableParameter.defaultValue;
        } else {
          controllableParameter.value = controllableParameter.defaultValue;
          controllingElement.value = controllableParameter.defaultValue;
        }
      }
    });
    window.electronAPI.parametersSendback(parameters);

    window.location.reload();
  }
});
function createControllableParameter(controllableParameter) {
  const form = document.getElementById("parameterContainer");
  let htmlToAppend = "";
  if (controllableParameter.type == "colour") {
    htmlToAppend +=
      "<h4>" +
      controllableParameter.title +
      "</h4><input type='text' class='reset-this form-control' id='" +
      controllableParameter.id +
      "' value='" +
      controllableParameter.value +
      "' data-coloris /><span class='badge rounded-pill' id='" +
      controllableParameter.id +
      "badge'" +
      ">Current colour</span>";
    form.insertAdjacentHTML("beforeend", htmlToAppend);
    const colour = document.getElementById(controllableParameter.id);
    const colourBadge = document.getElementById(
      controllableParameter.id + "badge"
    );
    colourBadge.style.backgroundColor = colour.value;
    colour.addEventListener("input", () => {
      colourBadge.style.backgroundColor = colour.value;
    });
    htmlToAppend = "<br /><br />";
  } else if (controllableParameter.type == "file") {
    htmlToAppend +=
      "<h4>" +
      controllableParameter.title +
      "</h4><button type='button' class='btn btn-outline-info' id='" +
      controllableParameter.id +
      "'>Open File</button>";
    form.insertAdjacentHTML("beforeend", htmlToAppend);
    const button = document.getElementById(controllableParameter.id);
    button.addEventListener("click", async () => {
      const filePath = await window.electronAPI.openFile();
      button.value = filePath;
    });
    htmlToAppend = "<br /><br />";
  } else if (controllableParameter.type == "range") {
    htmlToAppend +=
      "<h4>" +
      controllableParameter.title +
      "</h4><input type='range' class='form-range' min='" +
      controllableParameter.min +
      "' max='" +
      controllableParameter.max +
      "' step='" +
      controllableParameter.step +
      "' id='" +
      controllableParameter.id +
      "' value='" +
      controllableParameter.value +
      "' /><label for='" +
      controllableParameter.id +
      "' class='form-label' id='" +
      controllableParameter.id +
      "label'>Current value: " +
      controllableParameter.value +
      "</label>";
    form.insertAdjacentHTML("beforeend", htmlToAppend);
    const range = document.getElementById(controllableParameter.id);
    const rangeLabel = document.getElementById(
      controllableParameter.id + "label"
    );
    range.addEventListener("input", () => {
      rangeLabel.textContent = "Current value: " + range.value;
    });
    htmlToAppend = "<br /><br />";
  } else if (controllableParameter.type == "sensor") {
    appendSensors(controllableParameter);
  } else if (controllableParameter.type == "text") {
    htmlToAppend = `
        <h4>${controllableParameter.title}</h4><input type="text" class="form-control" id="${controllableParameter.id}" placeholder="${controllableParameter.defaultValue}" value="${controllableParameter.value}" />
        `;
    form.insertAdjacentHTML("beforeend", htmlToAppend);
    htmlToAppend = "<br />";
  } else if (controllableParameter.type == "bool") {
    const checkedVal = controllableParameter.value ? " checked" : "";
    htmlToAppend = `
    <div class="form-check form-switch">
      <input class="form-check-input" type="checkbox" role="switch" value="" id="${controllableParameter.id}"${checkedVal}>
      <label class="form-check-label" for="${controllableParameter.id}">
        ${controllableParameter.title}
      </label>
    </div>
    `;
  }
  parameters.push(controllableParameter);
  form.insertAdjacentHTML("beforeend", htmlToAppend);
}

function appendSensors(controllableParameter) {
  const form = document.getElementById("parameterContainer");
  let sensorObject = [undefined, undefined];
  let parentObject = [undefined, undefined];
  let superObject = [undefined, undefined];
  sensorinfo.forEach((object) => {
    object.sensorTree.forEach((object2) => {
      object2.sensors.forEach((object3) => {
        if (object3.path === controllableParameter.value) {
          sensorObject[0] = object3;
        }
      });
      if (sensorObject[0] != undefined) {
        parentObject[0] = object2;
        sensorObject[1] = sensorObject[0];
        sensorObject[0] = undefined;
      }
    });
    if (parentObject[0] != undefined) {
      superObject[1] = object;
      parentObject[1] = parentObject[0];
      parentObject[0] = undefined;
    }
  });
  sensorObject = sensorObject[1];
  parentObject = parentObject[1];
  superObject = superObject[1];

  let htmlToAppend = `
    <div class="sensorPicker" id="${controllableParameter.id}container">
    <h4>${controllableParameter.title}</h4>
    <label for="${controllableParameter.id}stage1">Pick device</label>
    <select class="form-select" id="${controllableParameter.id}stage1" onchange="updateStage1('${controllableParameter.id}')">
    </select>
    <label for="${controllableParameter.id}stage2">Pick sensor type</label>
    <select class="form-select" id="${controllableParameter.id}stage2" onchange="updateStage2('${controllableParameter.id}')">
    </select>
    <label for="${controllableParameter.id}">Pick sensor</label>
    <select class="form-select" id="${controllableParameter.id}">
    </select>
    </div>
    `;
  form.insertAdjacentHTML("beforeend", htmlToAppend);
  const superPicker = document.getElementById(
    controllableParameter.id + "stage1"
  );
  const parentPicker = document.getElementById(
    controllableParameter.id + "stage2"
  );
  const sensorPicker = document.getElementById(controllableParameter.id);
  sensorinfo.forEach((element) => {
    const newHTML =
      element == superObject
        ? `<option value="${element.path}" selected>${element.name}</option>`
        : `<option value="${element.path}">${element.name}</option>`;
    superPicker.insertAdjacentHTML("beforeend", newHTML);
  });
  updateStage1(controllableParameter.id);
  parentPicker.value = parentObject.category;
  updateStage2(controllableParameter.id);
  sensorPicker.value = sensorObject.path;
}

function updateStage1(domID) {
  const superPicker = document.getElementById(domID + "stage1");
  const parentPicker = document.getElementById(domID + "stage2");
  const sensorPicker = document.getElementById(domID);
  while (parentPicker.firstChild) {
    parentPicker.removeChild(parentPicker.lastChild);
  }
  while (sensorPicker.firstChild) {
    sensorPicker.removeChild(sensorPicker.lastChild);
  }
  sensorPicker.disabled = true;
  const pathToLookFor = superPicker.value;
  const newHTML = [];
  sensorinfo.forEach((element) => {
    if (element.path == pathToLookFor) {
      element.sensorTree.forEach((tree) => {
        newHTML.push(
          `<option value="${tree.category}">${tree.category}</option>`
        );
      });
    }
  });
  newHTML.forEach((elementDef) => {
    parentPicker.insertAdjacentHTML("beforeend", elementDef);
  });
  updateStage2(domID);
}

function updateStage2(domID) {
  const superPicker = document.getElementById(domID + "stage1");
  const parentPicker = document.getElementById(domID + "stage2");
  const sensorPicker = document.getElementById(domID);
  while (sensorPicker.firstChild) {
    sensorPicker.removeChild(sensorPicker.lastChild);
  }
  sensorPicker.disabled = false;
  const newHTML = [];
  sensorinfo.forEach((element) => {
    if (element.path === superPicker.value) {
      element.sensorTree.forEach((tree) => {
        if (parentPicker.value === tree.category) {
          tree.sensors.forEach((sensor) => {
            newHTML.push(
              `<option value="${sensor.path}">${sensor.name}</option>`
            );
          });
        }
      });
    }
  });
  newHTML.forEach((elementDef) => {
    sensorPicker.insertAdjacentHTML("beforeend", elementDef);
  });
}
