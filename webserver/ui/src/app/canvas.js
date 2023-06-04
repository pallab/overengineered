
import { num_range } from "./utils.js";
import { getLetterPixels } from "./letters.js";

 export const getFilledPixels = ( c, fillFraction ) => {
    // in pixels 
    const tileHeight = 10;
    const tileWidth = 8;
    const offColor = "#008000";
    const onColor = "#FF0000";

    const letterCords = getLetterPixels(c).sort((a,b) => b.y-a.y);
    const filledCords = letterCords.slice(0, Math.round(letterCords.length*fillFraction));

    const pixels = [];

    num_range(1, tileWidth-1).flatMap((x) => {
        num_range(1, tileHeight-1).flatMap((y) => {
            let fg = offColor;

            if(filledCords.find( (p) => p.x === x && p.y === y)) {
                fg = onColor;
            }
            pixels.push({ x : x, y: y, bd: true, bg : offColor, fg : fg});
        });
    })

    return pixels
}
