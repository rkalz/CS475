//
// Copyright 2019 <Rofael Aleezada>
//

#include <fstream>
#include <iterator>
#include <string>
#include <vector>

std::vector<unsigned char> bilinear_interpolation(std::vector<unsigned char>&,
    int, int);

int main() {
  std::string project_dir = PROJECT_DIR;

  std::ifstream input_hnd(project_dir + "/NN_brain_1024_1024.raw",
      std::ios::binary);
  std::vector<unsigned char> input_buffer(
      std::istreambuf_iterator<char>(input_hnd), {});
  input_hnd.close();

  auto output_buffer = bilinear_interpolation(input_buffer, 1024, 4);

  std::ofstream output_hnd(project_dir + "/NN_brain_4096_4096.raw",
      std::ios::binary);
  output_hnd.write(reinterpret_cast<const char*>(&output_buffer.at(0)),
      output_buffer.size());
  output_hnd.close();

  return 0;
}

std::vector<unsigned char> bilinear_interpolation(
    std::vector<unsigned char>& input_buffer, int input_dimension, int scale) {
      std::vector<unsigned char> output_buffer;

      // http://tech-algorithm.com/articles/bilinear-image-scaling/
      int output_dimenstion = input_dimension * scale;
      float ratio = ((float) (input_dimension - 1)) / output_dimenstion;

      for (int i = 0; i < output_dimenstion; i++) {
        for (int j = 0; j < output_dimenstion; j++) {
          int x = (int)(ratio * j);
          int y = (int)(ratio * i);
          float x_loss = (ratio * j) - x;
          float y_loss = (ratio * i) - y;

          int index = y * input_dimension + x;

          int a = input_buffer.at(index);
          int b = input_buffer.at(index + 1);
          int c = input_buffer.at(index + input_dimension);
          int d = input_buffer.at(index + input_dimension + 1);

          int interp_val = a * (1 - x_loss) * (1 - y_loss);
          interp_val += b * x_loss * (1 - y_loss);
          interp_val += c * y_loss * (1 - x_loss);
          interp_val += d * x_loss * y_loss;

          output_buffer.push_back(interp_val);
        }
      }

      return output_buffer;
}