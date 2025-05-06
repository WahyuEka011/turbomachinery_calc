#ifndef HOMEPAGEWINDOW_H
#define HOMEPAGEWINDOW_H

#include <QMainWindow>
#include "ui_homepagewindow.h"

QT_BEGIN_NAMESPACE
namespace Ui { class HomepageWindow; }
QT_END_NAMESPACE

class HomepageWindow : public QMainWindow
{
    Q_OBJECT

public:
    explicit HomepageWindow(QWidget *parent = nullptr);
    ~HomepageWindow();

private slots:
    void on_pushButtonLookup_clicked();
    void on_pushButtonANN_clicked();

private:
    Ui::HomepageWindow *ui;
};

#endif // HOMEPAGEWINDOW_H
