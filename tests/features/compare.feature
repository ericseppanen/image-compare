Feature: Image comparison

  Scenario Outline: Comparing an offset image to the original using RMS algorithm
    Given the images 'tests/data/pad_gaprao.png' and '<compare_image>' are loaded
    When comparing the images using RMS
    Then the similarity score is <result>

    Examples:
      | compare_image                           | result              |
      | tests/data/pad_gaprao.png               | 1.0                 |
      | tests/data/pad_gaprao_lighter.png       | 0.9201704590012584  |
      | tests/data/pad_gaprao_noise.png         | 0.7512383697679271  |
      | tests/data/pad_gaprao_gray_inverted.png | 0.497502556580533   |

  Scenario Outline: Comparing an offset image to the original using MSSIM algorithm
    Given the images 'tests/data/pad_gaprao.png' and '<compare_image>' are loaded
    When comparing the images using MSSIM
    Then the similarity score is <result>

    Examples:
      | compare_image                           | result              |
      | tests/data/pad_gaprao.png               | 1.0                 |
      | tests/data/pad_gaprao_lighter.png       | 0.9465500206208791  |
      | tests/data/pad_gaprao_noise.png         | 0.1260665609278695  |
      | tests/data/pad_gaprao_gray_inverted.png | -0.6559340036804088 |


    Scenario: Comparing an image to the original with RMS and checking the difference image
      Given the images 'tests/data/pad_gaprao.png' and 'tests/data/pad_gaprao_broken.png' are loaded
      When comparing the images using RMS
      Then the similarity image matches 'tests/data/pad_graparo_broken_rms_compare.png'

    Scenario: Comparing an image to the original with MSSIM and checking the difference image
      Given the images 'tests/data/pad_gaprao.png' and 'tests/data/pad_gaprao_broken.png' are loaded
      When comparing the images using MSSIM
      Then the similarity image matches 'tests/data/pad_graparo_broken_ssim_compare.png'
