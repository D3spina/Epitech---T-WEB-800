import './google/google'


document.addEventListener('DOMContentLoaded', async (event) => {

    try {
        const restaurants = await window.__TAURI__.invoke('get_data');
        const listeHtml = restaurants.map(r =>
            `<li>${r.nom}, ${r.adresse} - Cuisine ${r.cuisine}</li>`
        ).join('');
        document.getElementById('liste-restaurants').innerHTML = listeHtml;
    } catch (e) {
        console.error('Erreur lors de la récupération des données', e);
    }

    window.onload = function () {
        let googleMap = new Google();
    };

});