class Google {
  constructor() {
    try {
      this.map = this.initMap();
      this.directionsService = new google.maps.DirectionsService();
      this.directionsRenderer = new google.maps.DirectionsRenderer();
      this.directionsRenderer.setMap(this.map);
      this.citySet = new Set();
    } catch (error) {
      console.error('Failed to initialize Google Maps:', error);
    }
  }

  initMap() {
    try {
      let map = new google.maps.Map(document.getElementById('map'), {
        center: { lat: 48.68, lng: 6.2 },
        zoom: 10
      });
      console.log('Map initialized.');
      return map;
    } catch (error) {
      console.error('Map initialization failed:', error);
      return null;
    }
  }

  addMarker(lat, lng) {
    var location = { lat: lat, lng: lng };
    var marker = new google.maps.Marker({
      position: location,
      map: this.map,
    });
    marker.addListener('click', function() {
      alert('Marqueur cliqué');
    });
  }

  travelRoute(start, end, travelMode = 'DRIVING') {
    // La variable 'start' et 'end' peuvent être des objets {lat: , lng: } ou des chaînes d'adresse
    // this.citySet = new Set();
    this.directionsService.route({
      origin: start,
      destination: end,
      travelMode: travelMode
    }, (response, status) => {
      //const legs = route.legs
      //console.log(legs)
      if (status === 'OK') {
        this.directionsRenderer.setDirections(response);
        let route = response.routes[0];
        let legs = route.legs;
        legs.forEach(leg => {
          // Utiliser chaque début de segment pour vérifier la ville
          this.getCityFromCoords(leg.start_location);
          leg.steps.forEach(step => {
            this.getCityFromCoords(step.start_location);
          });
        });

      } else {
        console.error('Erreur dans la demande d\'itinéraire: ' + status);
      }
    });
  }

  getCityFromCoords(location) {
    let geocoder = new google.maps.Geocoder();
    geocoder.geocode({ 'location': location }, function(results, status) {
      if (status === 'OK' && results[0]) {
        let city = results.find(result => result.types.includes('locality'));
        if (city) {
          // Ajoute le nom de la ville au Set s'il n'est pas déjà présent
          this.citySet.add(city.formatted_address);
        } else {
          console.log('City not found for location:', location);
        }
      } else {
        console.error('Geocoder failed due to: ' + status);
      }
    });
    return
  }


}


