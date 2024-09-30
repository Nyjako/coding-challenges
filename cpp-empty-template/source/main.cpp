#include "cinder/app/App.h"
#include "cinder/app/RendererGl.h"
#include "cinder/gl/gl.h"
#include <cinder/GeomIo.h>
#include <cmath>

class TemplateAPP : public ci::app::App 
{
public:
    void setup() override;
    void update() override;
    void draw() override;

private:
    ci::Rectf m_Rect;
    float m_Rotation;
};

void TemplateAPP::setup() 
{
    float rectWidth  = 200.0f;
    float rectHeight = 100.0f;

     m_Rect = ci::Rectf(-rectWidth / 2, -rectHeight / 2, rectWidth / 2, rectHeight / 2);

    m_Rotation = 0.0f;
}

void TemplateAPP::update() 
{
    m_Rotation = fmodf((m_Rotation + 0.01f), 360.0f);
}

void TemplateAPP::draw() 
{
    // Clear the window
    ci::gl::clear(ci::Color(51 / 255.0f, 51 / 255.0f, 51 / 255.0f)); // Clear the window with black

    // Set the rectangle color to "whitesmoke"
    ci::gl::color(1.0f, 0.98f, 0.98f);

    // Save the state of the OpenGL matrix
    ci::gl::pushMatrices();

    // Translate to the center of the rectangle
    ci::gl::translate(getWindowWidth() / 2.0f, getWindowHeight() / 2.0f);

    // Rotate around the center
    ci::gl::rotate(m_Rotation);

    // Draw the rectangle (shifted back to original position)
    ci::gl::drawSolidRect(m_Rect);

    // Restore the OpenGL matrix state
    ci::gl::popMatrices();
}

CINDER_APP(TemplateAPP, cinder::app::RendererGl)
