#include "homepagewindow.h"
#include <QApplication>

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);
    HomepageWindow w;
    w.show();
    return a.exec();
}
