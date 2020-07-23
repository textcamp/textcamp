let checkSession = () => {
    // TODO: this is terrible 😂
    return document.cookie.startsWith("session")
}

// Scope holder. This is also gross.
// TODO: Refactor me
var socket = null;

if (checkSession()) {
    let proto = (document.location.protocol == 'https:' ? 'wss:' : 'ws:');
    socket = new WebSocket(`${proto}//${window.location.host}/ws/`);

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
        // axe the cookie
        document.cookie = "session=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
        // redirect to /
        window.location = "/";
    };
};

let showGameInterface = () => {
    let si = document.getElementById("signin-interface");
    si.style.display = "none";
    let gi = document.getElementById("game-interface");
    gi.style.display = "block";
}

let showSigninInterface = () => {
    let gi = document.getElementById("game-interface");
    gi.style.display = "none";
    let si = document.getElementById("signin-interface");
    si.style.display = "block";
}

let route = (json) => {
    if ('time' in json) {
        showTime(json.time);
        return;
    }

    if ('info' in json) {
        showInfo(json.info);
        return;
    }

    if ('error' in json) {
        record('error', json.error);
        return;
    }

    if ('combat' in json) {
        showCombat(json.combat);
        return;
    }

    if ('inventory' in json) {
        showInventory(json.inventory);
        return;
    }

    if ('space' in json) {
        showSpace(json.space);
        return;
    }

    if ('item' in json) {
        // take over the main description
        showSpace(json.item);
        return;
    }

    if ('character' in json) {
        // take over the main description
        showSpace(json.character);
        return;
    }

    if ('population' in json) {
        showPopulation(json.population);
        return;
    }

    if ('health' in json) {
        showHealth(json.health);
        return;
    }
}


let record = (target, str) => {
    let elementId = `${target}-content`;
    let element = document.getElementById(elementId);
    element.innerHTML = str;
}

let enterKey = (e) => {
    if (event.key === 'Enter') {
        socket.send(e.value);
        e.value = "";
        record('error', '');
    }
}

let showInfo = (info) => {
    showTemporary(info);
}

let showCombat = (combat) => {
    showTemporary(combat);
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
    if (health === 0) {
        record('health', "💀".repeat(5));
        return;
    }

    let red_hearts = Math.round(health / 20);
    let black_hearts = 5 - red_hearts;
    record('health', `${"❤️".repeat(red_hearts)}${"🖤".repeat(black_hearts)}`);
}

let doAction = (action) => {
    socket.send(action);
    record('error', '');
}

let asAction = (label, action) => {
    return `<span class="action" title="Action: ${action}" onclick="doAction('${action}'); return false;">${label}</span>`;
}

let showTemporary = (text) => {
    let elementId = "temporary-messages";
    let messages = document.getElementById(elementId);

    var item = document.createElement("li");
    let id = randomIdentifier();
    item.id = id;
    item.innerText = text;

    messages.appendChild(item);
    setTimeout(() => { fade(id); }, 1000);
}

let randomIdentifier = () => {
    return Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15)
}

let fade = (id) => {
    let target = document.getElementById(id);
    target.classList.add("fade");
    setTimeout(() => { remove(id); }, 3000);
}

let remove = (id) => {
    let target = document.getElementById(id);
    target.remove();
}