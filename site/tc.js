let route = (json) => {
    if (json.time) {
        showTime(json.time);
        return;
    }

    if (json.info) {
        showInfo(json.info);
        return;
    }

    if (json.exits) {
        showExits(json.exits);
        return;
    }

    if (json.attributes) {
        showAttributes(json.attributes);
        return;
    }

    if (json.error) {
        record('error', json.error);
        return;
    }

    if (json.combat) {
        record('combat', json.combat);
        return;
    }

    if (json.inventory) {
        showInventory(json.inventory);
        return;
    }

    if (json.space) {
        showSpace(json.space);
        return;
    }

    if (json.item) {
        record('info', json.item.text);
        return;
    }

    if (json.character) {
        showCharacter(json.character);
        return;
    }
}


let record = (target, str) => {
    let elementId = `${target}-content`;
    let element = document.getElementById(elementId);
    element.innerHTML = str;
}

let socket = new WebSocket(`ws://${window.location.host}/ws/`);
window.addEventListener("beforeunload", e => { socket.close() });

socket.onopen = (e) => {
    socket.send("refresh");
};

socket.onmessage = (e) => {
    let json = JSON.parse(e.data);
    console.log(json);
    route(json);
};

socket.onclose = (e) => {
    console.log("Socket closed.")
    record("error", "Connection closed.");

};

socket.onerror = (e) => {
    console.log("Socket error!");
    record("error", "Error connecting to the server!");
}

let enterKey = (e) => {
    if (event.key === 'Enter') {
        socket.send(e.value);
        e.value = "";
    }
}

let fadeInfo = () => {
    let elementId = `info-content`;
    let element = document.getElementById(elementId);
    element.style.opacity = "0";
}

let showInfo = (info) => {
    let elementId = `info-content`;
    let element = document.getElementById(elementId);
    element.innerHTML = info;
    element.style.opacity = "1";

    setTimeout(fadeInfo, 3000);
}

let showAttributes = (attributes) => {
    let attrs = `in: ${attributes.intelligence} ws: ${attributes.wisdom} con: ${attributes.constitution} dx: ${attributes.dexterity} ch: ${attributes.charisma} st: ${attributes.strength}`;

    record('attributes', attrs);
}

let showSpace = (space) => {
    var long_description = space.text;
    Object.keys(space.clicks).forEach(
        (key) => {
            let orig_text = `[[${key}]]`;
            let new_text = asAction(key, space.clicks[key]);
            long_description = long_description.replace(orig_text, new_text);
        }
    );

    let description = long_description.trim();

    record('space', description);
}

let showExits = (exits) => {
    var output = "";
    exits.forEach((direction) => output += asAction(direction, `go ${direction}`));

    record('exits', output);
}

let showCharacter = (character) => {
    record('info', character.long_description);
}

let showInventory = (inventory) => {
    let output = "<ul>";
    inventory.forEach(i => { output += `<li>${i}</li>` });
    output += "</ul>";

    record('inventory', output);
}

let showTime = (time) => {
    let output = `${time.hour}:${time.minute} day: ${time.day} month: ${time.month} year: ${time.year}`;
    record('time', output);
}

let doAction = (action) => {
    socket.send(action);
}

let asAction = (label, action) => {
    return `<span class="action" onclick="doAction('${action}'); return false;">${label}</span>`;
}