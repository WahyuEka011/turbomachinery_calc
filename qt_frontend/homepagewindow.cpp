#include "homepagewindow.h"
#include "ui_homepagewindow.h"
#include "mainwindow.h"
#include "annwindow.h"

HomepageWindow::HomepageWindow(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::HomepageWindow)
{
    ui->setupUi(this);
}

HomepageWindow::~HomepageWindow()
{
    delete ui;
}

void HomepageWindow::on_pushButtonLookup_clicked()
{
    MainWindow *lookupWindow = new MainWindow();
    lookupWindow->show();
    this->close();  // tutup homepage kalau mau
}

void HomepageWindow::on_pushButtonANN_clicked()
{
    AnnWindow *annWindow = new AnnWindow();
    annWindow->show();
    this->close();  // tutup homepage kalau mau
}
