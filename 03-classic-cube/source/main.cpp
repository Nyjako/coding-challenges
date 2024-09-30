#include "cinder/app/App.h"
#include "cinder/app/RendererGl.h"
#include "cinder/gl/gl.h"
#include "cinder/CameraUi.h"
#include <cinder/Vector.h>
#include <cinder/gl/wrapper.h>

using namespace ci;
using namespace ci::app;

class RotatingCube : public App 
{
public:
    void setup() override;
    void update() override;
    void draw() override;

private:
    gl::BatchRef m_Cube;
    gl::GlslProgRef m_Shader;
    CameraPersp m_Cam;
    CameraUi m_CamUi;

    float m_RotationAngle = 0.0f;
};

void RotatingCube::setup() 
{
    // Load and compile shaders
    m_Shader = gl::GlslProg::create(loadAsset("shaders/cube.vert"), loadAsset("shaders/cube.frag"));

    // Create cube mesh
    auto cube = geom::Cube().subdivisions(1);
    m_Cube = gl::Batch::create(cube, m_Shader);

    // camera setup
    m_Cam.lookAt(vec3(3, 3, 3), vec3(0));
    m_CamUi = CameraUi(&m_Cam, getWindow());

    gl::enableDepthWrite();
    gl::enableDepthRead();
}

void RotatingCube::update() 
{
    m_RotationAngle += 0.01f;
}

void RotatingCube::draw() 
{
    gl::clear(Color(0, 0, 0));

    // Rotation
    gl::setMatrices(m_Cam);
    gl::rotate(m_RotationAngle, vec3(1, 1, 0));

    // draw cube
    m_Cube->draw();
}

CINDER_APP(RotatingCube, RendererGl)
