#include "cinder/app/App.h"
#include "cinder/app/RendererGl.h"
#include "cinder/gl/gl.h"
#include "cinder/CameraUi.h"
#include <cinder/Vector.h>
#include <cinder/gl/wrapper.h>
#include <glm/fwd.hpp>
#include <glm/gtx/quaternion.hpp>

class RotatingCube : public ci::app::App 
{
public:
    void setup() override;
    void update() override;
    void draw() override;

private:
    ci::gl::BatchRef m_Cube;
    ci::gl::GlslProgRef m_Shader;
    ci::CameraPersp m_Cam;
    ci::CameraUi m_CamUi;

    float m_RotationAngle = 0.0f;
};

void RotatingCube::setup() 
{
    // Load and compile shaders
    try {
        m_Shader = ci::gl::GlslProg::create(loadAsset("shaders/cube.vert"), loadAsset("shaders/cube.frag"));
    } catch (const ci::gl::GlslProgCompileExc& ex) {
        console() << "Shader compile error: " << ex.what() << std::endl;
        quit();
    }

    // Create cube mesh
    auto cube = ci::geom::Cube().subdivisions(1);
    m_Cube = ci::gl::Batch::create(cube, m_Shader);

    // camera setup
    m_Cam.lookAt(ci::vec3(3, 3, 3), ci::vec3(0));
    m_CamUi = ci::CameraUi(&m_Cam, getWindow());

    ci::gl::enableDepthWrite();
    ci::gl::enableDepthRead();
}

void RotatingCube::update() 
{
    m_RotationAngle += 0.01f;
}

void RotatingCube::draw() 
{
    ci::gl::clear(ci::Color(0, 0, 0));

    ci::gl::setMatrices(m_Cam);

    glm::mat4 modelMatrix = glm::rotate(m_RotationAngle, glm::vec3(1, 1, 0));

    m_Shader->uniform("uModelViewProjection", m_Cam.getProjectionMatrix() * m_Cam.getViewMatrix() * modelMatrix);

    m_Cube->draw();
}

CINDER_APP(RotatingCube, cinder::app::RendererGl)
