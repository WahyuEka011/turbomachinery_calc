#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow {
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private slots:
    void on_calculateButton_clicked();
    void on_homeButton_clicked();
    void drawGraph();      

private:
    Ui::MainWindow *ui;
};

extern "C" double lookup_value(double x);  // Fungsi Rust
#endif // MAINWINDOW_H
