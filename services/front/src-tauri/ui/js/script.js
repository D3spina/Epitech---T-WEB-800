document.addEventListener('DOMContentLoaded', async (event) => {
  const { invoke } = window.__TAURI__.tauri;

  window.initMap = function() {
    window.googleMapInstance = new Google(); // Crée et stocke l'instance globalement
  };

  await loadRestaurants(invoke, "Nancy", 1000);

  document.getElementById('search').addEventListener('click', async function() {
    const departValue = document.getElementById('depart').value;
    const arriveValue = document.getElementById('arrivee').value;

    if (!departValue || !arriveValue) {
      alert("Veuillez vérifier vos villes");
      return;
    } else {
      setupRoutes(departValue, arriveValue);

    }

  });
});





async function loadRestaurants(invoke, ville, ratio) {
  try {
    // Attendre la résolution de la promesse pour obtenir les données des restaurants
    invoke('get_restaurants', { ville: ville, ratio: ratio })
      .then((response) => {
        const restaurants = response;
        //const restaurants = await window.__TAURI__.invoke('get_restaurants', { ville: "Nancy", ratio: 1000 });
        const dataList = document.getElementById('data-list');
        dataList.innerHTML = '';

        if (restaurants && Array.isArray(restaurants)) {
          restaurants.forEach(restaurant => {
            const restaurantDiv = document.createElement('div');
            restaurantDiv.className = 'restaurant';

            const infoDiv = document.createElement('div');

            const infoDivRatingName = document.createElement('div')

            const imageContainer = document.createElement('div');

            const name = document.createElement('h3');
            name.textContent = restaurant.name;
            name.className = 'name';

            const image = document.createElement('img');
            image.src = restaurant.picture;

            const rating = document.createElement('p');
            rating.textContent = parseFloat(restaurant.rating.toFixed(1));
            rating.className = "rating";

            const address = document.createElement('p');
            address.textContent = restaurant.address;
            address.className = 'address'

            const addImage = document.createElement('img');
            addImage.src = './assets/add.png'
            addImage.className = "addImage";


            imageContainer.appendChild(image)


            infoDivRatingName.appendChild(name);
            infoDivRatingName.appendChild(rating);
            infoDivRatingName.appendChild(imageContainer);

            infoDivRatingName.className = "topContainer";
            infoDiv.appendChild(address);
            infoDiv.appendChild(addImage);
            restaurantDiv.appendChild(infoDivRatingName)
            restaurantDiv.appendChild(infoDiv);
            dataList.appendChild(restaurantDiv);
          });
        }
      })

  } catch (error) {
    console.error('Failed to load restaurants', error);
  }
}



async function getLoc(invoke, ville) {
  try {
    const response = await invoke('get_localisation', { ville: ville });
    return response;
  } catch (err) {
    console.error('Failed to fetch location:', err);
    throw err;
  }
}

function setupRoutes(villeDepart, villeArrive) {
  if (window.googleMapInstance) {
    window.googleMapInstance.travelRoute(villeDepart, villeArrive);
  } else {
    setTimeout(setupRoutes, 100);
  }
}

