document.addEventListener('DOMContentLoaded', async (event) => {

    async function loadRestaurants() {
        try {
            // Attendre la résolution de la promesse pour obtenir les données des restaurants
            const restaurants = await window.__TAURI__.invoke('get_restaurants');

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


                    imageContainer.appendChild(image)


                    infoDivRatingName.appendChild(name);
                    infoDivRatingName.appendChild(rating);
                    infoDivRatingName.appendChild(imageContainer);

                    infoDivRatingName.className = "topContainer";

                    infoDiv.appendChild(address);
                    restaurantDiv.appendChild(infoDivRatingName)
                    restaurantDiv.appendChild(infoDiv);
                    dataList.appendChild(restaurantDiv);
                });
            }
        } catch (error) {
            console.error('Failed to load restaurants', error);
        }
    }

    await loadRestaurants();

});
