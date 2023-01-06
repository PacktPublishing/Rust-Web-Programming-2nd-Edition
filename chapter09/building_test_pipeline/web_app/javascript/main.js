

function renderItems(items, processType, elementId, processFunction) {
    let itemsMeta = [];
    let placeholder = "<div>"

    for (let i = 0; i < items.length; i++) {
        let title = items[i]["title"];
        let placeholderId = processType +
            "-" + title.replaceAll(" ", "-");

        placeholder += '<div class="itemContainer">' +
            '<p>' + title + '</p>' +
            '<div class="actionButton" ' +
            'id="' + placeholderId + '">'
            + processType + '</div>' + "</div>";
        itemsMeta.push({"id": placeholderId, "title": title});
    }
    placeholder += "</div>"
    document.getElementById(elementId).innerHTML = placeholder;

    for (let i = 0; i < itemsMeta.length; i++) {
        document.getElementById(itemsMeta[i]["id"]).addEventListener("click", processFunction);
    }
}


function apiCall(url, method) {
    let xhr = new XMLHttpRequest();
    xhr.withCredentials = true;
    xhr.addEventListener('readystatechange', function() {
        if (this.readyState === this.DONE) {
            renderItems(JSON.parse(this.responseText)["pending_items"],
                "edit", "pendingItems", editItem);
            renderItems(JSON.parse(this.responseText)["done_items"], "delete",
                "doneItems", deleteItem);
            document.getElementById("completeNum").innerHTML = JSON.parse(
                this.responseText)["done_item_count"];
            document.getElementById("pendingNum").innerHTML = JSON.parse(
                this.responseText)["pending_item_count"];
        }
    })
    xhr.open(method, url);
    xhr.setRequestHeader('content-type', 'application/json');
    xhr.setRequestHeader('user-token', 'token');
    return xhr
}


function editItem() {
    let title = this.id.replaceAll("-", " ")
        .replace("edit ", "");
    let call = apiCall("/v1/item/edit", "POST");

    let json = {
        "title": title,
        "status": "DONE"
    };
    call.send(JSON.stringify(json));
}


function deleteItem() {
    let title = this.id.replaceAll("-", " ")
        .replace("delete ", "");
    let call = apiCall("/v1/item/delete", "POST");

    let json = {
        "title": title,
        "status": "DONE"
    };

    call.send(JSON.stringify(json));
}

function getItems() {
    let call = apiCall("/v1/item/get", 'GET');
    call.send()
}

getItems();

document.getElementById("create-button")
    .addEventListener("click", createItem);

function createItem() {
    let title = document.getElementById("name");
    let call = apiCall("/v1/item/create/" +
        title.value, "POST");
    call.send();
    document.getElementById("name").value = null;
}
