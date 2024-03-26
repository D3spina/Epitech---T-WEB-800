class Google {
    constructor() {
        this.map = this.initMap();
    }

    initMap() {
        let map = new google.maps.Map(document.getElementById('map'), {
            center: {lat: 48.68, lng: 6.2},
            zoom: 8
        });
        return map;
    }
}

// S'assurer que cette fonction est globalement accessible si utilisée comme callback
window.initMap = function() {
    let googleMap = new Google();
}
