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

    if (json.error) {
        record('error', json.error);
        return;
    }

    if (json.combat) {
        showCombat(json.combat);
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
        // take over the main description
        showSpace(json.item);
        return;
    }

    if (json.character) {
        // take over the main description
        showSpace(json.character);
        return;
    }

    if (json.population) {
        showPopulation(json.population);
        return;
    }

    if (json.health) {
        showHealth(json.health);
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
        record('error', '');
    }
}

let showInfo = (info) => {
    let elementId = `info-content`;
    let element = document.getElementById(elementId);
    element.innerHTML = element.innerHTML + info + "\n";
    element.style.opacity = "1";

}

let showCombat = (combat) => {
    let elementId = `combat-content`;
    let element = document.getElementById(elementId);
    element.innerHTML = element.innerHTML + combat + "\n";
    element.style.opacity = "1";

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

let showInventory = (inventory) => {
    let output = "<ul>";
    inventory.forEach(i => { output += `<li>${i}</li>` });
    output += "</ul>";

    record('inventory', output);
}

let showPopulation = (population) => {
    let output = "<ul>"
    population.forEach(i => { output += `<li>${i}</li>` });
    output += "</ul>";

    record('population', output);
}

let showTime = (time) => {
    let output = `${time.hour}:${time.minute} day: ${time.day} month: ${time.month} year: ${time.year}`;
    record('time', output);
}

let showHealth = (health) => {
    let red_hearts = Math.round(health / 20);
    let black_hearts = 5 - red_hearts;
    record('health', `${"❤️".repeat(red_hearts)}${"🖤".repeat(black_hearts)}`);
}

let doAction = (action) => {
    socket.send(action);
    record('error', '');
}

let asAction = (label, action) => {
    return `<span class="action" onclick="doAction('${action}'); return false;">${label}</span>`;
}