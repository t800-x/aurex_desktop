export enum Section {
    search = 1,
    songs,
    albums,
    artists
}

export enum RightPaneContent {
    queue = 1,
    lyrics
}

class Router {
    current = $state<Section>(Section.songs);
    rightPaneContent = $state<RightPaneContent | null>(null);
    rightPaneOverlaying = $state(false);

    constructor() {}

    go(s: Section): void {
        this.current = s;
        console.log("Current Section: " + this.current);
    }

    setRightPaneContent(content: RightPaneContent | null): void {
        if (content == this.rightPaneContent) {
            this.rightPaneContent = null;
            return;
        }

        this.rightPaneContent = content;
    }
}

export const router = new Router();