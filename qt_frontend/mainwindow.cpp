#include "mainwindow.h"
#include "ui_mainwindow.h"
#include <QString>
#include <QPixmap>
#include <QProcess>
#include <QFile>
#include <QPainter>
#include <QDebug>
#include <cmath>


extern "C" {
    double get_sin_from_lookup(double angle);
    double get_cos_from_lookup(double angle);
    void draw_plot();
}

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent), ui(new Ui::MainWindow) {
    ui->setupUi(this);
    connect(ui->calculateButton, &QPushButton::clicked,
            this, &MainWindow::on_calculateButton_clicked);

    // ✅ Connect tombol HOME pada Page Taylor dan Lookup Table
    connect(ui->homeButton, &QPushButton::clicked,
        this, &MainWindow::on_homeButton_clicked);

    // Memuat gambar statis ke dalam label saat aplikasi dimulai
    QPixmap pixmap(":/spanduk2/Downloads/42.png");
    if (!pixmap.isNull()) {
        ui->plotLabel->setPixmap(pixmap.scaled(ui->plotLabel->size(), Qt::KeepAspectRatio));
    } else {
        qDebug() << "Failed to load image!";
    }
}

MainWindow::~MainWindow() {
    delete ui;
}

void MainWindow::on_calculateButton_clicked() {                             // FUNGSI BUTTON CALCULATE PADA MENU TAYLOR DAN LOOKUP
    double input = ui->inputField->text().toDouble();
    double sin_result = get_sin_from_lookup(input);
    double cos_result = get_cos_from_lookup(input);

    QString hasil = "Sin: " + QString::number(sin_result) +
                    "\nCos: " + QString::number(cos_result);

    ui->resultLabel->setText(hasil);

    // Jalankan proses plot (bisa panggil Rust juga kalau fungsi plot ada di sana)
    QProcess::execute("target/debug/rust_backend_plot", QStringList() << QString::number(input));

    // Tampilkan hasil plot
    QPixmap plot("output.png");
    ui->plotLabel->setPixmap(plot.scaled(ui->plotLabel->size(), Qt::KeepAspectRatio));


    draw_plot();  // Fungsi dari Rust yang menggambar grafik

    // Panggil fungsi Rust untuk menggambar grafik dan menyimpan ke file (misalnya output.png)
    draw_plot();  // Fungsi Rust yang menggambar grafik

    // Setelah menggambar grafik, tampilkan gambar di QLabel
    QPixmap pixmap("output.png");  // Pastikan output.png adalah hasil dari draw_plot()

    // Pastikan gambar berhasil dimuat
    if (!pixmap.isNull()) {
        ui->plotLabel->setPixmap(pixmap.scaled(ui->plotLabel->size(), Qt::KeepAspectRatio));  // Sesuaikan ukuran QLabel
    } else {
        qDebug() << "Failed to load the image!";
    }
}

void MainWindow::on_homeButton_clicked() {
    this->close();  // atau bisa emit sinyal untuk kembali ke main menu kalau pakai multiwindow
}

//FUNGSI CHART LOOKUP
void MainWindow::drawGraph() {
    // Panggil fungsi Rust untuk menggambar grafik ke file PNG
    draw_plot();  // Rust handle semua pakai plotters dan simpan file PNG

    // Setelah grafik digambar di Rust, load hasilnya ke Qt
    QPixmap pixmap("output.png");  // Ganti nama file jika perlu
    ui->plotLabel->setPixmap(pixmap.scaled(ui->plotLabel->size(), Qt::KeepAspectRatio));
}