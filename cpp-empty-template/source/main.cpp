#include "cinder/app/App.h"
#include "cinder/app/RendererGl.h"
#include "cinder/gl/gl.h"
#include <cinder/GeomIo.h>
#include <cmath>

using namespace ci;
using namespace ci::app;
using namespace std;

class TemplateAPP : public App 
{
public:
    void setup() override;
    void update() override;
    void draw() override;

private:
    Rectf m_Rect;
    float m_Rotation;
};

void TemplateAPP::setup() 
{
    float rectWidth  = 200.0f;
    float rectHeight = 100.0f;

     m_Rect = Rectf(-rectWidth / 2, -rectHeight / 2, rectWidth / 2, rectHeight / 2);

    m_Rotation = 0.0f;
}

void TemplateAPP::update() 
{
    m_Rotation = fmodf((m_Rotation + 0.01f), 360.0f);
}

void TemplateAPP::draw() 
{
    // Clear the window
    gl::clear(Color(51 / 255.0f, 51 / 255.0f, 51 / 255.0f)); // Clear the window with black

    // Set the rectangle color to "whitesmoke"
    gl::color(1.0f, 0.98f, 0.98f);

    // Save the state of the OpenGL matrix
    gl::pushMatrices();

    // Translate to the center of the rectangle
    gl::translate(getWindowWidth() / 2.0f, getWindowHeight() / 2.0f);

    // Rotate around the center
    gl::rotate(m_Rotation);

    // Draw the rectangle (shifted back to original position)
    gl::drawSolidRect(m_Rect);

    // Restore the OpenGL matrix state
    gl::popMatrices();
}

CINDER_APP(TemplateAPP, RendererGl)
