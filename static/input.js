function sendCardForm() {
    let form = document.getElementById("card_form");
    let result = document.getElementsByClassName("result")[0];
    let fd = new Object();

    if (result.classList.contains("success")) {
        result.classList.remove("success");
    }
    if (result.classList.contains("failure")) {
        result.classList.remove("failure");
    }

    if (!(form.card_name.value == "")) {
        fd.name = form.name.value;
    }
    // This will always exist due to the form not showing up without a card set in the DB
    fd.card_set = form.card_set.value;
    if (!(form.card_set.value == "")) {
        fd.rarity = form.card_set.value;
    }
    fd.rarity = form.rarity.value;
    if (!(form.card_number.value == "")) {
        fd.card_number = form.card_number.value;
    }
    fd.red = form.red.checked;
    fd.blue = form.blue.checked;
    fd.black = form.black.checked;
    fd.green = form.green.checked;
    fd.white = form.white.checked;
    fd.colorless = form.colorless.checked;
    if (!(form.cmc.value == "")) {
        fd.cmc = form.cmc.value;
    }

    // Converts Object to urlencoded string
    // RESOURCE - stackoverflow.com/questions/9909620/convert-json-into-uri-encoded-string
    let data = new URLSearchParams(fd);

    let xhr = new XMLHttpRequest();
    xhr.addEventListener("load", function(event) {
        // alert(event.target.responseText);
        form.reset();
        result.innerHTML = "Success - Card into DB";
        result.classList.add("success");
    });
    xhr.addEventListener("error", function(event) {
        result.innerHTML = "Something went wrong";
        result.classList.add("failure");
    });
    xhr.open("POST", "/receive_card");
    xhr.setRequestHeader("Content-Type", "application/x-www-form-urlencoded");
    xhr.send(data.toString());

    form.name.focus();
}

document.getElementById("card_form").addEventListener("submit", function(event) {
    event.preventDefault();
    sendCardForm();

});