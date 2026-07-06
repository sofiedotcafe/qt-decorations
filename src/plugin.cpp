#include "bridge.h"
#include <QtWaylandClient/private/qwaylandabstractdecoration_p.h>
#include <QtGui/QPainter>
#include <QtGui/QPaintDevice>
#include <QtGui/QWindow>

static PaintCallback on_paint = nullptr;
static MarginsCallback on_margins = nullptr;
static MouseCallback on_mouse = nullptr;
static TouchCallback on_touch = nullptr;

extern "C" void register_rust_vtable(PaintCallback p, MarginsCallback m, MouseCallback ms,
                                     TouchCallback t)
{
    on_paint = p;
    on_margins = m;
    on_mouse = ms;
    on_touch = t;
}

extern "C" void fill_rect(void *painter_ptr, int x, int y, int w, int h, int r, int g, int b)
{
    if (painter_ptr) {
        static_cast<QPainter *>(painter_ptr)->fillRect(x, y, w, h, QColor(r, g, b));
    }
}

class CatppuccinWaylandDecoration : public QtWaylandClient::QWaylandAbstractDecoration
{
public:
    CatppuccinWaylandDecoration() : QtWaylandClient::QWaylandAbstractDecoration() { }

    void paint(QPaintDevice *device) override
    {
        if (!device)
            return;

        QPainter painter(device);
        int w = device->width();

        if (on_paint)
            on_paint(&painter, w, 49);
    }

#if QT_VERSION >= 0x060000
    QMargins margins(MarginsType) const override{
#else
    QMargins margins() const override
    {
#endif
        int l = 0, t = 0, r = 0, b = 0;
    if (on_margins)
        on_margins(false, &l, &t, &r, &b);
    return QMargins(l, t, r, b);
}

    bool handleMouse(QtWaylandClient::QWaylandInputDevice*, const QPointF &local, const QPointF&, Qt::MouseButtons b, Qt::KeyboardModifiers) override
{
    int w = (window()) ? window()->width() : 800;
    return on_mouse ? on_mouse(w, local.x(), local.y(), static_cast<int>(b)) : false;
}

#if QT_VERSION >= 0x060000
bool handleTouch(QtWaylandClient::QWaylandInputDevice *, const QPointF &local, const QPointF &,
                 QEventPoint::State state, Qt::KeyboardModifiers) override
{
    int w = (window()) ? window()->width() : 800;
    return on_touch ? on_touch(w, local.x(), local.y(), state == 1 || state == 2) : false;
}
#else
    bool handleTouch(QtWaylandClient::QWaylandInputDevice *, const QPointF &local, const QPointF &,
                     Qt::TouchPointState state, Qt::KeyboardModifiers) override
    {
        int w = (window()) ? window()->width() : 800;
        return on_touch ? on_touch(w, local.x(), local.y(), state == 1 || state == 2) : false;
    }
#endif
}
;