#ifndef ANNWINDOW_H
#define ANNWINDOW_H

#include <QWidget>
#include "ui_annwindow.h"
#include "ui_annwindow.h"

class AnnWindow : public QMainWindow
{
    Q_OBJECT

public:
    explicit AnnWindow(QWidget *parent = nullptr);
    ~AnnWindow();

private slots:
    void on_buttonPredict_clicked();
    void on_homeButtonann_clicked();

private:
    Ui::AnnWindow *ui;
    QString run_ann_from_backend(double electricity, double rpm, double steam, double vibration);
};

extern "C" {
    const char* run_ann_from_qt(double electricity, double rpm, double pressure, double vibration);
}

#endif // ANNWINDOW_H