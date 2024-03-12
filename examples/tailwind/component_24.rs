use rust_web_ui::*;

pub struct Example24;

pub struct Example24Props {}

impl Default for Example24Props {
	fn default() -> Example24Props {
		Example24Props { }
	}
}

impl Widget<'_> for Example24 {
	type Props = Example24Props;

	fn render(mut ctx: Ctx<'_>, props: Example24Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[Style::PaddingTop(24)]);
				
				props.child("1", Dynamic).run("nav", |props| {
					props.set_attribute("aria-label", "Breadcrumb");
					
					props.child("1", Dynamic).run("ol", |props| {
						props.set_attribute("role", "list");
						props.styles(&[NoStyle::Noop("mx-auto"), Style::Flex, NoStyle::Noop("max-w-2xl"), Style::ItemsCenter, Style::SpaceX(8), Style::PaddingX(16), Screen::Small(&[Style::PaddingX(24)]), Screen::Large(&[NoStyle::Noop("max-w-7xl")]), Screen::Large(&[Style::PaddingX(32)])]);
						
						props.child("1", Dynamic).run("li", |props| {
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[Style::Flex, Style::ItemsCenter]);
								
								props.child("1", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::MarginRight(8), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Men"));
								});
								props.child("3", Icon).run(|props| {
									props.style(&[Style::Width(20), Style::Width(16), Style::TextColor(Color::Fg(33))]);
								});
							});
						});
						props.child("3", Dynamic).run("li", |props| {
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[Style::Flex, Style::ItemsCenter]);
								
								props.child("1", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::MarginRight(8), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Clothing"));
								});
								props.child("3", Icon).run(|props| {
									props.style(&[Style::Width(20), Style::Width(16), Style::TextColor(Color::Fg(33))]);
								});
							});
						});
						props.child("5", Dynamic).run("li", |props| {
							props.styles(&[NoStyle::Noop("text-sm")]);
							
							props.child("1", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.set_attribute("aria-current", "page");
								props.styles(&[NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(56)), Action::Hover(&[Style::TextColor(Color::Fg(67))])]);
								
								props.child("0", Label).run(|props| props.set_text("Basic Tee 6-Pack"));
							});
						});
					});
				});
				//  Image gallery 
				props.child("5", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), Style::MarginTop(24), NoStyle::Noop("max-w-2xl"), Screen::Small(&[Style::PaddingX(24)]), Screen::Large(&[NoStyle::Noop("grid")]), Screen::Large(&[NoStyle::Noop("max-w-7xl")]), Screen::Large(&[NoStyle::Noop("grid-cols-3")]), Screen::Large(&[NoStyle::Noop("gap-x-8")]), Screen::Large(&[Style::PaddingX(32)])]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("aspect-h-4"), NoStyle::Noop("aspect-w-3"), NoStyle::Noop("hidden"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Screen::Large(&[Style::Block])]);
						
						props.child("1", Dynamic).run("img", |props| {
							props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-02-secondary-product-shot.jpg");
							props.set_attribute("alt", "Two each of gray, white, and black shirts laying flat.");
							props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
						});
					});
					props.child("3", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("hidden"), Screen::Large(&[NoStyle::Noop("grid")]), Screen::Large(&[NoStyle::Noop("grid-cols-1")]), Screen::Large(&[NoStyle::Noop("gap-y-8")])]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("aspect-h-2"), NoStyle::Noop("aspect-w-3"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-02-tertiary-product-shot-01.jpg");
								props.set_attribute("alt", "Model wearing plain black basic tee.");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("aspect-h-2"), NoStyle::Noop("aspect-w-3"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg")]);
							
							props.child("1", Dynamic).run("img", |props| {
								props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-02-tertiary-product-shot-02.jpg");
								props.set_attribute("alt", "Model wearing plain gray basic tee.");
								props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
							});
						});
					});
					props.child("5", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("aspect-h-5"), NoStyle::Noop("aspect-w-4"), Screen::Large(&[NoStyle::Noop("aspect-h-4")]), Screen::Large(&[NoStyle::Noop("aspect-w-3")]), Screen::Small(&[NoStyle::Noop("overflow-hidden")]), Screen::Small(&[NoStyle::Noop("rounded-lg")])]);
						
						props.child("1", Dynamic).run("img", |props| {
							props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-page-02-featured-product-shot.jpg");
							props.set_attribute("alt", "Model wearing plain white basic tee.");
							props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
						});
					});
				});
				//  Product info 
				props.child("9", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-2xl"), Style::PaddingX(16), Style::PaddingBottom(64), Style::PaddingTop(40), Screen::Small(&[Style::PaddingX(24)]), Screen::Large(&[NoStyle::Noop("grid")]), Screen::Large(&[NoStyle::Noop("max-w-7xl")]), Screen::Large(&[NoStyle::Noop("grid-cols-3")]), Screen::Large(&[NoStyle::Noop("grid-rows-[auto,auto,1fr]")]), Screen::Large(&[NoStyle::Noop("gap-x-8")]), Screen::Large(&[Style::PaddingX(32)]), Screen::Large(&[Style::PaddingBottom(96)]), Screen::Large(&[Style::PaddingTop(64)])]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[Screen::Large(&[NoStyle::Noop("col-span-2")]), Screen::Large(&[NoStyle::Noop("border-r")]), Screen::Large(&[NoStyle::Noop("border-gray-200")]), Screen::Large(&[Style::PaddingRight(32)])]);
						
						props.child("1", Dynamic).run("h1", |props| {
							props.styles(&[NoStyle::Noop("text-2xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-3xl")])]);
							
							props.child("0", Label).run(|props| props.set_text("Basic Tee 6-Pack"));
						});
					});
					//  Options 
					props.child("5", Dynamic).run("div", |props| {
						props.styles(&[Style::MarginTop(16), Screen::Large(&[NoStyle::Noop("row-span-3")]), Screen::Large(&[Style::MarginTop(0)])]);
						
						props.child("1", Dynamic).run("h2", |props| {
							props.styles(&[NoStyle::Noop("sr-only")]);
							
							props.child("0", Label).run(|props| props.set_text("Product information"));
						});
						props.child("3", Dynamic).run("p", |props| {
							props.styles(&[NoStyle::Noop("text-3xl"), NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("$192"));
						});
						//  Reviews 
						props.child("7", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(24)]);
							
							props.child("1", Dynamic).run("h3", |props| {
								props.styles(&[NoStyle::Noop("sr-only")]);
								
								props.child("0", Label).run(|props| props.set_text("Reviews"));
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[Style::Flex, Style::ItemsCenter]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::TextColor(Color::Fg(100)), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0")]);
									});
									props.child("3", Icon).run(|props| {
										props.style(&[Style::TextColor(Color::Fg(100)), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0")]);
									});
									props.child("5", Icon).run(|props| {
										props.style(&[Style::TextColor(Color::Fg(100)), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0")]);
									});
									props.child("7", Icon).run(|props| {
										props.style(&[Style::TextColor(Color::Fg(100)), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0")]);
									});
									props.child("9", Icon).run(|props| {
										props.style(&[Style::TextColor(Color::Fg(22)), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0")]);
									});
								});
								props.child("3", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.set_text("4 out of 5 stars"));
								});
								props.child("5", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("text-indigo-500")])]);
									
									props.child("0", Label).run(|props| props.set_text("117 reviews"));
								});
							});
						});
						props.child("9", Dynamic).run("form", |props| {
							props.styles(&[Style::MarginTop(40)]);
							
							//  Colors 
							props.child("3", Dynamic).run("div", |props| {
								props.child("1", Dynamic).run("h3", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text("Color"));
								});
								props.child("3", Dynamic).run("fieldset", |props| {
									props.set_attribute("x-data", "window.Components.radioGroup({ initialCheckedIndex: 0 })");
									props.set_attribute("x-init", "init()");
									props.styles(&[Style::MarginTop(16)]);
									
									props.child("1", Dynamic).run("legend", |props| {
										props.styles(&[NoStyle::Noop("sr-only")]);
										
										props.child("0", Label).run(|props| props.set_text("Choose a color"));
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[Style::Flex, Style::ItemsCenter, Style::SpaceX(12)]);
										
										props.child("1", Dynamic).run("label", |props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("-m-0.5"), Style::Flex, NoStyle::Noop("cursor-pointer"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-full"), Style::Padding(2), Action::Hover(&[NoStyle::Noop("outline-none")]), NoStyle::Noop("ring-gray-400")]);
											props.set_attribute("x-description", r#"Active and Checked: "ring ring-offset-1"	Not Active and Checked: "ring-2""#);
											props.set_attribute(":class", "{ 'ring ring-offset-1': (active === 'White') && (value === 'White'), 'undefined': !(active === 'White') || !(value === 'White'), 'ring-2': !(active === 'White') && (value === 'White'), 'undefined': (active === 'White') || !(value === 'White') }");
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "color-choice");
												props.set_attribute("value", "White");
												props.styles(&[NoStyle::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "color-choice-0-label");
											});
											props.child("3", Dynamic).run("span", |props| {
												props.set_attribute("id", "color-choice-0-label");
												props.styles(&[NoStyle::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.set_text("White"));
											});
											props.child("5", Dynamic).run("span", |props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::Width(32), Style::Width(32), NoStyle::Noop("bg-white"), NoStyle::Noop("rounded-full"), NoStyle::Noop("border"), NoStyle::Noop("border-black"), NoStyle::Noop("border-opacity-10")]);
											});
										});
										props.child("3", Dynamic).run("label", |props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("-m-0.5"), Style::Flex, NoStyle::Noop("cursor-pointer"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-full"), Style::Padding(2), Action::Hover(&[NoStyle::Noop("outline-none")]), NoStyle::Noop("ring-gray-400")]);
											props.set_attribute("x-description", r#"Active and Checked: "ring ring-offset-1"	Not Active and Checked: "ring-2""#);
											props.set_attribute(":class", "{ 'ring ring-offset-1': (active === 'Gray') && (value === 'Gray'), 'undefined': !(active === 'Gray') || !(value === 'Gray'), 'ring-2': !(active === 'Gray') && (value === 'Gray'), 'undefined': (active === 'Gray') || !(value === 'Gray') }");
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "color-choice");
												props.set_attribute("value", "Gray");
												props.styles(&[NoStyle::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "color-choice-1-label");
											});
											props.child("3", Dynamic).run("span", |props| {
												props.set_attribute("id", "color-choice-1-label");
												props.styles(&[NoStyle::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.set_text("Gray"));
											});
											props.child("5", Dynamic).run("span", |props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::Width(32), Style::Width(32), Style::Color(Color::Fg(22)), NoStyle::Noop("rounded-full"), NoStyle::Noop("border"), NoStyle::Noop("border-black"), NoStyle::Noop("border-opacity-10")]);
											});
										});
										props.child("5", Dynamic).run("label", |props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("-m-0.5"), Style::Flex, NoStyle::Noop("cursor-pointer"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-full"), Style::Padding(2), Action::Hover(&[NoStyle::Noop("outline-none")]), NoStyle::Noop("ring-gray-900")]);
											props.set_attribute("x-description", r#"Active and Checked: "ring ring-offset-1"	Not Active and Checked: "ring-2""#);
											props.set_attribute(":class", "{ 'ring ring-offset-1': (active === 'Black') && (value === 'Black'), 'undefined': !(active === 'Black') || !(value === 'Black'), 'ring-2': !(active === 'Black') && (value === 'Black'), 'undefined': (active === 'Black') || !(value === 'Black') }");
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "color-choice");
												props.set_attribute("value", "Black");
												props.styles(&[NoStyle::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "color-choice-2-label");
											});
											props.child("3", Dynamic).run("span", |props| {
												props.set_attribute("id", "color-choice-2-label");
												props.styles(&[NoStyle::Noop("sr-only")]);
												
												props.child("0", Label).run(|props| props.set_text("Black"));
											});
											props.child("5", Dynamic).run("span", |props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[Style::Width(32), Style::Width(32), Style::Color(Color::Fg(100)), NoStyle::Noop("rounded-full"), NoStyle::Noop("border"), NoStyle::Noop("border-black"), NoStyle::Noop("border-opacity-10")]);
											});
										});
									});
								});
							});
							//  Sizes 
							props.child("7", Dynamic).run("div", |props| {
								props.styles(&[Style::MarginTop(40)]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween]);
									
									props.child("1", Dynamic).run("h3", |props| {
										props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
										
										props.child("0", Label).run(|props| props.set_text("Size"));
									});
									props.child("3", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("text-indigo-500")])]);
										
										props.child("0", Label).run(|props| props.set_text("Size guide"));
									});
								});
								props.child("3", Dynamic).run("fieldset", |props| {
									props.set_attribute("x-data", "window.Components.radioGroup({ initialCheckedIndex: 2 })");
									props.set_attribute("x-init", "init()");
									props.styles(&[Style::MarginTop(16)]);
									
									props.child("1", Dynamic).run("legend", |props| {
										props.styles(&[NoStyle::Noop("sr-only")]);
										
										props.child("0", Label).run(|props| props.set_text("Choose a size"));
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-4"), NoStyle::Noop("gap-4"), Screen::Small(&[NoStyle::Noop("grid-cols-8")]), Screen::Large(&[NoStyle::Noop("grid-cols-4")])]);
										
										props.child("1", Dynamic).run("label", |props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), Screen::Small(&[Style::PaddingY(24)]), NoStyle::Noop("cursor-not-allowed"), Style::Color(Color::Fg(6)), Style::TextColor(Color::Fg(22))]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'XXS'), 'undefined': !(active === 'XXS') }");
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "XXS");
												props.set_attribute("disabled", "");
												props.styles(&[NoStyle::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-0-label");
											});
											props.child("3", Dynamic).run("span", |props| {
												props.set_attribute("id", "size-choice-0-label");
												
												props.child("0", Label).run(|props| props.set_text("XXS"));
											});
											props.child("5", Dynamic).run("span", |props| {
												props.set_attribute("aria-hidden", "true");
												props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("absolute"), NoStyle::Noop("-inset-px"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-2"), NoStyle::Noop("border-gray-200")]);
												
												props.child("1", Icon).run(|props| {
													props.style(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("stroke-2"), Style::TextColor(Color::Fg(22))]);
												});
											});
										});
										props.child("3", Dynamic).run("label", |props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), Screen::Small(&[Style::PaddingY(24)]), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("bg-white"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm")]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'XS'), 'undefined': !(active === 'XS') }");
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "XS");
												props.styles(&[NoStyle::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-1-label");
											});
											props.child("3", Dynamic).run("span", |props| {
												props.set_attribute("id", "size-choice-1-label");
												
												props.child("0", Label).run(|props| props.set_text("XS"));
											});
											props.child("5", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("absolute"), NoStyle::Noop("-inset-px"), NoStyle::Noop("rounded-md")]);
												props.set_attribute("aria-hidden", "true");
												props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
												props.set_attribute(":class", "{ 'border': (active === 'XS'), 'border-2': !(active === 'XS'), 'border-indigo-500': (value === 'XS'), 'border-transparent': !(value === 'XS') }");
											});
										});
										props.child("5", Dynamic).run("label", |props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), Screen::Small(&[Style::PaddingY(24)]), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("bg-white"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm")]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'S'), 'undefined': !(active === 'S') }");
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "S");
												props.styles(&[NoStyle::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-2-label");
											});
											props.child("3", Dynamic).run("span", |props| {
												props.set_attribute("id", "size-choice-2-label");
												
												props.child("0", Label).run(|props| props.set_text("S"));
											});
											props.child("5", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("absolute"), NoStyle::Noop("-inset-px"), NoStyle::Noop("rounded-md")]);
												props.set_attribute("aria-hidden", "true");
												props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
												props.set_attribute(":class", "{ 'border': (active === 'S'), 'border-2': !(active === 'S'), 'border-indigo-500': (value === 'S'), 'border-transparent': !(value === 'S') }");
											});
										});
										props.child("7", Dynamic).run("label", |props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), Screen::Small(&[Style::PaddingY(24)]), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("bg-white"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm")]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'M'), 'undefined': !(active === 'M') }");
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "M");
												props.styles(&[NoStyle::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-3-label");
											});
											props.child("3", Dynamic).run("span", |props| {
												props.set_attribute("id", "size-choice-3-label");
												
												props.child("0", Label).run(|props| props.set_text("M"));
											});
											props.child("5", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("absolute"), NoStyle::Noop("-inset-px"), NoStyle::Noop("rounded-md")]);
												props.set_attribute("aria-hidden", "true");
												props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
												props.set_attribute(":class", "{ 'border': (active === 'M'), 'border-2': !(active === 'M'), 'border-indigo-500': (value === 'M'), 'border-transparent': !(value === 'M') }");
											});
										});
										props.child("9", Dynamic).run("label", |props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), Screen::Small(&[Style::PaddingY(24)]), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("bg-white"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm")]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'L'), 'undefined': !(active === 'L') }");
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "L");
												props.styles(&[NoStyle::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-4-label");
											});
											props.child("3", Dynamic).run("span", |props| {
												props.set_attribute("id", "size-choice-4-label");
												
												props.child("0", Label).run(|props| props.set_text("L"));
											});
											props.child("5", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("absolute"), NoStyle::Noop("-inset-px"), NoStyle::Noop("rounded-md")]);
												props.set_attribute("aria-hidden", "true");
												props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
												props.set_attribute(":class", "{ 'border': (active === 'L'), 'border-2': !(active === 'L'), 'border-indigo-500': (value === 'L'), 'border-transparent': !(value === 'L') }");
											});
										});
										props.child("11", Dynamic).run("label", |props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), Screen::Small(&[Style::PaddingY(24)]), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("bg-white"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm")]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'XL'), 'undefined': !(active === 'XL') }");
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "XL");
												props.styles(&[NoStyle::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-5-label");
											});
											props.child("3", Dynamic).run("span", |props| {
												props.set_attribute("id", "size-choice-5-label");
												
												props.child("0", Label).run(|props| props.set_text("XL"));
											});
											props.child("5", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("absolute"), NoStyle::Noop("-inset-px"), NoStyle::Noop("rounded-md")]);
												props.set_attribute("aria-hidden", "true");
												props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
												props.set_attribute(":class", "{ 'border': (active === 'XL'), 'border-2': !(active === 'XL'), 'border-indigo-500': (value === 'XL'), 'border-transparent': !(value === 'XL') }");
											});
										});
										props.child("13", Dynamic).run("label", |props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), Screen::Small(&[Style::PaddingY(24)]), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("bg-white"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm")]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === '2XL'), 'undefined': !(active === '2XL') }");
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "2XL");
												props.styles(&[NoStyle::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-6-label");
											});
											props.child("3", Dynamic).run("span", |props| {
												props.set_attribute("id", "size-choice-6-label");
												
												props.child("0", Label).run(|props| props.set_text("2XL"));
											});
											props.child("5", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("absolute"), NoStyle::Noop("-inset-px"), NoStyle::Noop("rounded-md")]);
												props.set_attribute("aria-hidden", "true");
												props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
												props.set_attribute(":class", "{ 'border': (active === '2XL'), 'border-2': !(active === '2XL'), 'border-indigo-500': (value === '2XL'), 'border-transparent': !(value === '2XL') }");
											});
										});
										props.child("15", Dynamic).run("label", |props| {
											props.set_attribute("x-radio-group-option", "");
											props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), Screen::Small(&[Style::PaddingY(24)]), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("bg-white"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm")]);
											props.set_attribute("x-state:on:size.instock", "In Stock");
											props.set_attribute("x-state:off:size.instock", "Out of Stock");
											props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
											props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === '3XL'), 'undefined': !(active === '3XL') }");
											
											props.child("1", Dynamic).run("input", |props| {
												props.set_attribute("type", "radio");
												props.set_attribute("x-model", "value");
												props.set_attribute("name", "size-choice");
												props.set_attribute("value", "3XL");
												props.styles(&[NoStyle::Noop("sr-only")]);
												props.set_attribute("aria-labelledby", "size-choice-7-label");
											});
											props.child("3", Dynamic).run("span", |props| {
												props.set_attribute("id", "size-choice-7-label");
												
												props.child("0", Label).run(|props| props.set_text("3XL"));
											});
											props.child("5", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("absolute"), NoStyle::Noop("-inset-px"), NoStyle::Noop("rounded-md")]);
												props.set_attribute("aria-hidden", "true");
												props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
												props.set_attribute(":class", "{ 'border': (active === '3XL'), 'border-2': !(active === '3XL'), 'border-indigo-500': (value === '3XL'), 'border-transparent': !(value === '3XL') }");
											});
										});
									});
								});
							});
							props.child("9", Dynamic).run("button", |props| {
								props.set_attribute("type", "submit");
								props.styles(&[Style::MarginTop(40), Style::Flex, NoStyle::Noop("w-full"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), NoStyle::Noop("border-transparent"), NoStyle::Noop("bg-indigo-600"), Style::PaddingX(32), Style::PaddingY(12), NoStyle::Noop("text-base"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-white"), Action::Hover(&[NoStyle::Noop("bg-indigo-700")]), Action::Hover(&[NoStyle::Noop("outline-none")]), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-indigo-500")]), Action::Hover(&[NoStyle::Noop("ring-offset-2")])]);
								
								props.child("0", Label).run(|props| props.set_text("Add to bag"));
							});
						});
					});
					props.child("7", Dynamic).run("div", |props| {
						props.styles(&[Style::PaddingY(40), Screen::Large(&[NoStyle::Noop("col-span-2")]), Screen::Large(&[NoStyle::Noop("col-start-1")]), Screen::Large(&[NoStyle::Noop("border-r")]), Screen::Large(&[NoStyle::Noop("border-gray-200")]), Screen::Large(&[Style::PaddingBottom(64)]), Screen::Large(&[Style::PaddingRight(32)]), Screen::Large(&[Style::PaddingTop(24)])]);
						
						//  Description and details 
						props.child("3", Dynamic).run("div", |props| {
							props.child("1", Dynamic).run("h3", |props| {
								props.styles(&[NoStyle::Noop("sr-only")]);
								
								props.child("0", Label).run(|props| props.set_text("Description"));
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[Style::SpaceY(24)]);
								
								props.child("1", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-base"), Style::TextColor(Color::Fg(100))]);
									
									props.child("0", Label).run(|props| props.set_text(r#"The Basic Tee 6-Pack allows you to fully express your vibrant personality with three grayscale options. Feeling adventurous? Put on a heather gray tee. Want to be a trendsetter? Try our exclusive colorway: "Black". Need to add an extra pop of color to your outfit? Our white tee has you covered."#));
								});
							});
						});
						props.child("5", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(40)]);
							
							props.child("1", Dynamic).run("h3", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Highlights"));
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[Style::MarginTop(16)]);
								
								props.child("1", Dynamic).run("ul", |props| {
									props.set_attribute("role", "list");
									props.styles(&[NoStyle::Noop("list-disc"), Style::SpaceY(8), Style::PaddingLeft(16), NoStyle::Noop("text-sm")]);
									
									props.child("1", Dynamic).run("li", |props| {
										props.styles(&[Style::TextColor(Color::Fg(44))]);
										
										props.child("0", Dynamic).run("span", |props| {
											props.styles(&[Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.set_text("Hand cut and sewn locally"));
										});
									});
									props.child("3", Dynamic).run("li", |props| {
										props.styles(&[Style::TextColor(Color::Fg(44))]);
										
										props.child("0", Dynamic).run("span", |props| {
											props.styles(&[Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.set_text("Dyed with our proprietary colors"));
										});
									});
									props.child("5", Dynamic).run("li", |props| {
										props.styles(&[Style::TextColor(Color::Fg(44))]);
										
										props.child("0", Dynamic).run("span", |props| {
											props.styles(&[Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.set_text("Pre-washed & pre-shrunk"));
										});
									});
									props.child("7", Dynamic).run("li", |props| {
										props.styles(&[Style::TextColor(Color::Fg(44))]);
										
										props.child("0", Dynamic).run("span", |props| {
											props.styles(&[Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.set_text("Ultra-soft 100% cotton"));
										});
									});
								});
							});
						});
						props.child("7", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(40)]);
							
							props.child("1", Dynamic).run("h2", |props| {
								props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("0", Label).run(|props| props.set_text("Details"));
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[Style::MarginTop(16), Style::SpaceY(24)]);
								
								props.child("1", Dynamic).run("p", |props| {
									props.styles(&[NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(67))]);
									
									props.child("0", Label).run(|props| props.set_text(r#"The 6-Pack includes two black, two white, and two heather gray Basic Tees. Sign up for our subscription service and be the first to get new, exciting colors, like our upcoming "Charcoal Gray" limited release."#));
								});
							});
						});
					});
				});
			});
		});
	}
}
