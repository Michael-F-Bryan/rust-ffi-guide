#include <QtWidgets/QMainWindow>
#include <QtWidgets/QPushButton>

class MainWindow : public QMainWindow {
  Q_OBJECT

public:
  explicit MainWindow(QWidget *parent = 0);
  virtual ~MainWindow(){};
private slots:
  void onClick();

private:
  QPushButton *button;
};
