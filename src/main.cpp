#include <iostream>
#include <SFML/Graphics.hpp>
#include "Car.hpp"
#include "Checkerboard.hpp"

int main() {
	sf::RenderWindow window(sf::VideoMode{ sf::Vector2u{800, 600} }, "Stehendes Lauflicht");

	sf::Texture texture;
	assert(texture.loadFromFile("../../../images/car.png"));

	Car car(texture);
	Checkerboard checkerboard{
		.primary = sf::Color::Black,
		.secondary = sf::Color::White,
		.debug_primary = sf::Color(50, 50, 50),
		.debug_secondary = sf::Color(200, 200, 200),
		.scale = 120.f
	};

	auto last_update = std::chrono::high_resolution_clock::now();
	while (window.isOpen()) {
		auto now = std::chrono::high_resolution_clock::now();
		float delta = std::chrono::duration_cast<std::chrono::duration<float>>(now - last_update).count();
		last_update = now;

		sf::Event sf_event;
		while (window.pollEvent(sf_event)) {
			if (sf_event.type == sf::Event::Closed || (sf_event.type == sf::Event::KeyPressed && sf_event.key.code == sf::Keyboard::Escape))
				window.close();
		}

		car.Update(delta, checkerboard);

		window.clear(sf::Color::Blue);
		checkerboard.Draw(window);
		car.Draw(window);
		window.display();
	}
}
