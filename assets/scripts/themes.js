window.addEventListener("load", (event) => {
    window.electronAPI.getThemeList();
})

const body = document.getElementById("card-container");

window.electronAPI.receiveThemeList((_event, theme) => {
    const htmlToAppend = "<div class='card'><img src='" + theme.preview + "' class='card-img' />" + "<h5 class='card-title'>" + theme.title + "</h5><p class='card-text'>" + theme.description + "</p></div></div>";
    body.insertAdjacentHTML("beforeend", htmlToAppend);
});