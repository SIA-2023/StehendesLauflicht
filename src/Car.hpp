#pragma once

#include <SFML/Graphics.hpp>
#include "Checkerboard.hpp"
#include <numbers>
#include <array>

float radians(float degrees) {
    return degrees * (std::numbers::pi_v<float> / 180.f);
}

float degrees(float radians) {
    return radians * (180.0 / std::numbers::pi_v<float>);
}

struct Light {
    sf::CircleShape circle;
    sf::Vector2f offset;
};

class Car {
public:
    Car(const sf::Texture& texture) : m_Sprite(texture) {
        m_Sprite.setOrigin({ m_Sprite.getLocalBounds().width / 2.f, m_Sprite.getLocalBounds().height / 2.f });
        m_Sprite.setScale({ 0.075f, 0.15f });

        for (auto& circle : m_LightCircles) {
            circle.setFillColor(sf::Color::Yellow);
            circle.setRadius(7.5f);
            circle.setOrigin({ circle.getRadius(), circle.getRadius() });
        }
    }

	void Update(float delta, const Checkerboard& checkerboard) {
        float leftMotor = 0.f, rightMotor = 0.f;

        if (sf::Keyboard::isKeyPressed(sf::Keyboard::W))
            leftMotor = rightMotor = k_move_speed;
        else if (sf::Keyboard::isKeyPressed(sf::Keyboard::S))
            leftMotor = rightMotor = -k_move_speed;
        
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::A))
            leftMotor = 0.f;
        else if (sf::Keyboard::isKeyPressed(sf::Keyboard::D))
            rightMotor = 0.f;
        
        float delta_orientation = ((leftMotor - rightMotor) / k_wheel_distance) * delta;
        m_Sprite.rotate(sf::radians(delta_orientation));
        m_Orientation += delta_orientation;
        float velocity = (leftMotor + rightMotor) / 2.f;
        m_Sprite.move(sf::Vector2f{ std::cos(m_Orientation), std::sin(m_Orientation) } * velocity * delta);
        
        // update lights
        sf::Vector2f carPosition = m_Sprite.getPosition();
        sf::Angle carRotation = m_Sprite.getRotation();
        for (int i = 0; i < m_LightCircles.size(); i++) {
            m_LightCircles[i].setPosition(carPosition + m_LightOffsets[i].rotatedBy(carRotation));
            m_LightCircles[i].setFillColor(checkerboard.GetColor(m_LightCircles[i].getPosition()));
        }
    }

    void Draw(sf::RenderWindow& window) const {
        window.draw(m_Sprite);
        for (const auto& circle: m_LightCircles)
            window.draw(circle);
    }

private:
    sf::Sprite m_Sprite;
    float m_Orientation = 0.f;
    std::array<sf::CircleShape, 24> m_LightCircles;
    std::array<sf::Vector2f, 24> m_LightOffsets {
        // vorne
        sf::Vector2f{60.f, -80.f},
        sf::Vector2f{36.6f, -80.f},
        sf::Vector2f{13.3f, -80.f},
        sf::Vector2f{-13.3f, -80.f},
        sf::Vector2f{-36.6f, -80.f},
        sf::Vector2f{-60.f, -80.f},

        // hinten
        sf::Vector2f{60.f, 90.f},
        sf::Vector2f{36.6f, 90.f},
        sf::Vector2f{13.3f, 90.f},
        sf::Vector2f{-13.3f, 90.f},
        sf::Vector2f{-36.6f, 90.f},
        sf::Vector2f{-60.f, 90.f},

        // links
        sf::Vector2f{-80.f, 70.f},
        sf::Vector2f{-80.f, 44.f},
        sf::Vector2f{-80.f, 20.f},
        sf::Vector2f{-80.f, -8.f},
        sf::Vector2f{-80.f, -34.f},
        sf::Vector2f{-80.f, -60.f},

        // rechts
        sf::Vector2f{80.f, 70.f},
        sf::Vector2f{80.f, 44.f},
        sf::Vector2f{80.f, 20.f},
        sf::Vector2f{80.f, -8.f},
        sf::Vector2f{80.f, -34.f},
        sf::Vector2f{80.f, -60.f},
    };

	constexpr static float k_move_speed = 150;
    constexpr static float k_wheel_distance = 160.f;
};