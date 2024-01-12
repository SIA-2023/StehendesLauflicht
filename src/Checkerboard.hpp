#pragma once

#include <SFML/Graphics.hpp>

struct Checkerboard {
	sf::Color primary, secondary;
	sf::Color debug_primary, debug_secondary;
	float scale = 1.f;

	void Draw(sf::RenderWindow& window) const {
		auto[width, height] = window.getSize();
		for (float y = 0; y < (float)height; y += scale) {
			for (float x = 0; x < (float)width; x += scale) {
				sf::RectangleShape rect{ { scale, scale } };
				rect.setPosition({ x, y });
				rect.setFillColor(GetColor(rect.getPosition(), true));
				window.draw(rect);
			}
		}
	}

	sf::Color GetColor(const sf::Vector2f& position, bool debug = false) const {
		sf::Vector2f scaledPos = position / scale;
		if (debug)
			return (((int)scaledPos.x + (int)scaledPos.y) % 2 == 0) ? debug_primary : debug_secondary;
		else
			return (((int)scaledPos.x + (int)scaledPos.y) % 2 == 0) ? primary : secondary;
	}
};