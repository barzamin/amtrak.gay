import L from "leaflet";
import amtk from './amtk';

const map = L.map('map').setView([40.82, -73.96], 13);
const tileLayer = L.tileLayer('https://stamen-tiles.a.ssl.fastly.net/toner-lite/{z}/{x}/{y}.png', {
    attribution: 'Map tiles by <a href="http://stamen.com">Stamen Design</a>, under <a href="http://creativecommons.org/licenses/by/3.0">CC BY 3.0</a>. Data by <a href="http://openstreetmap.org">OpenStreetMap</a>, under <a href="http://www.openstreetmap.org/copyright">ODbL</a>.',
    maxZoom: 20,
    minZoom: 0,
});
tileLayer.addTo(map);

const uris = {
    trainData: 'https://maps.amtrak.com/services/MapDataService/trains/getTrainsData',
};

(async () => {
    const encData = await fetch(uris.trainData).then(r => r.text()));
    const geoJSON = amtk.decrypt(encData);
    console.log(geoJSON);
})();
