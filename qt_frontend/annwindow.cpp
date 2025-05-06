#include "annwindow.h"
#include "ui_annwindow.h"
#include <QPixmap>
#include <QMessageBox>
#include <QString>
#include <QDebug>
#include <cstdlib>
#include <cstring>

extern "C" {
    const char* run_ann_from_qt(double electricity, double rpm, double steam, double vibration);
}

extern "C" void free_string(const char*);

AnnWindow::AnnWindow(QWidget *parent) :
    QMainWindow(parent), ui(new Ui::AnnWindow)
{
    ui->setupUi(this);  // Inisialisasi UI
}

AnnWindow::~AnnWindow()
{
    delete ui;
}

void AnnWindow::on_buttonPredict_clicked()
{
    bool ok1, ok2, ok3, ok4;

    double electricity = ui->inputElectricity->text().toDouble(&ok1);
    double rpm         = ui->inputRpm->text().toDouble(&ok2);
    double steam       = ui->inputSteam->text().toDouble(&ok3);
    double vibration   = ui->inputVibration->text().toDouble(&ok4);

    if (!ok1 || !ok2 || !ok3 || !ok4) {
        QMessageBox::warning(this, "Input Error", "Masukkan angka yang valid di semua kolom!");
        return;
    }

    const char* result = run_ann_from_qt(electricity, rpm, steam, vibration);

    // Convert C-string result to QString
    QString result_q = QString::fromUtf8(result);
    free_string(result);  // supaya tidak leak

    ui->hasilTerminal->setText("Prediksi sedang diproses...");
    ui->hasilPrediksi->setText(result_q);

    QPixmap graf("output.png");
    ui->tempatGrafikann->setPixmap(graf.scaled(300, 300, Qt::KeepAspectRatio));
}

void AnnWindow::on_homeButtonann_clicked()
{
    this->hide();
    // Tambahkan logika kembali ke MainWindow kalau diperlukan
}

QString AnnWindow::run_ann_from_backend(double electricity, double rpm, double steam, double vibration)
{
    const char* result_c = run_ann_from_qt(electricity, rpm, steam, vibration);
    QString result = QString::fromUtf8(result_c);
    free((void*)result_c);  // jika alokasi dari Rust
    return result;
}
