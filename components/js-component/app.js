// Exports required by mod-world
export function init() {
    console.log("JS component initialized");
}

export function actorInit(actor, x, y) {
    console.log(`JS actor init: x=${x}, y=${y}`);
    actor.setX(x);
    actor.setY(y);
}

export function actorUpdate(actor) {
    console.log("JS actor update");
}

export function actorRender(actor) {
    console.log("JS actor render");
}