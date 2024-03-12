use rust_web_ui::*;

pub struct Example18;

pub struct Example18Props {}

impl Default for Example18Props {
	fn default() -> Example18Props {
		Example18Props { }
	}
}

impl Widget<'_> for Example18 {
	type Props = Example18Props;

	fn render(mut ctx: Ctx<'_>, props: Example18Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[Style::Flex, NoStyle::Noop("min-h-[700px]")]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[Style::Flex, NoStyle::Noop("w-full"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("bg-white"), Style::PaddingX(16), Screen::Small(&[Style::PaddingX(24)]), Screen::Large(&[Style::PaddingX(32)])]);
				props.set_attribute("style", "height: 700px");
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[Style::Width(288)]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.set_attribute("x-data", "{ open: true }");
						props.set_attribute("@keydown.window.escape", "open = false");
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, NoStyle::Noop("flex-col-reverse")]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[Style::MarginTop(16), Style::Flex, Style::ItemsCenter, Style::JustifyBetween, Style::SpaceX(32), NoStyle::Noop("text-base"), NoStyle::Noop("font-medium"), Style::TextColor(Color::Fg(100))]);
								
								props.child("1", Dynamic).run("h3", |props| {
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										
										props.child("0", Label).run(|props| props.set_text("Basic Tee 6-Pack"));
									});
								});
								props.child("3", Dynamic).run("p", |props| {
									props.child("0", Label).run(|props| props.set_text("$192"));
								});
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("aspect-h-3"), NoStyle::Noop("aspect-w-4"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(11))]);
								
								props.child("1", Dynamic).run("img", |props| {
									props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-quick-preview-02-preview.jpg");
									props.set_attribute("alt", "Model wearing gray t-shirt.");
									props.styles(&[NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
								});
								props.child("3", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, NoStyle::Noop("items-end"), Style::Padding(16)]);
									
									props.child("1", Dynamic).run("button", |props| {
										props.set_attribute("type", "button");
										props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("z-10"), NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), NoStyle::Noop("bg-opacity-75"), Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("opacity-0"), Action::Hover(&[NoStyle::Noop("opacity-100")]), NoStyle::NoopGroup("group-hover", NoStyle::Noop("opacity-100"))]);
										props.set_attribute("@click", "open = true");
										
										props.child("0", Label).run(|props| props.set_text("Quick View"));
										props.child("1", Dynamic).run("span", |props| {
											props.styles(&[NoStyle::Noop("sr-only")]);
											
											props.child("0", Label).run(|props| props.set_text(", Basic Tee 6-Pack"));
										});
									});
								});
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.set_attribute("x-show", "open");
							props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("z-10")]);
							props.set_attribute("x-ref", "dialog");
							props.set_attribute("aria-modal", "true");
							
							props.child("1", Dynamic).run("div", |props| {
								props.set_attribute("x-show", "open");
								props.set_attribute("x-transition:enter", "ease-out duration-300");
								props.set_attribute("x-transition:enter-start", "opacity-0");
								props.set_attribute("x-transition:enter-end", "opacity-100");
								props.set_attribute("x-transition:leave", "ease-in duration-200");
								props.set_attribute("x-transition:leave-start", "opacity-100");
								props.set_attribute("x-transition:leave-end", "opacity-0");
								props.set_attribute("x-description", "Background backdrop, show/hide based on modal state.");
								props.styles(&[NoStyle::Noop("fixed"), NoStyle::Noop("inset-0"), NoStyle::Noop("hidden"), Style::Color(Color::Fg(56)), NoStyle::Noop("bg-opacity-75"), NoStyle::Noop("transition-opacity"), Screen::Medium(&[Style::Block])]);
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("fixed"), NoStyle::Noop("inset-0"), NoStyle::Noop("z-10"), NoStyle::Noop("w-screen"), NoStyle::Noop("overflow-y-auto")]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, NoStyle::Noop("min-h-full"), NoStyle::Noop("items-stretch"), Style::JustifyCenter, NoStyle::Noop("text-center"), Screen::Medium(&[Style::ItemsCenter]), Screen::Medium(&[Style::PaddingX(8)]), Screen::Large(&[Style::PaddingX(16)])]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.set_attribute("x-show", "open");
										props.set_attribute("x-transition:enter", "ease-out duration-300");
										props.set_attribute("x-transition:enter-start", "opacity-0 translate-y-4 md:translate-y-0 md:scale-95");
										props.set_attribute("x-transition:enter-end", "opacity-100 translate-y-0 md:scale-100");
										props.set_attribute("x-transition:leave", "ease-in duration-200");
										props.set_attribute("x-transition:leave-start", "opacity-100 translate-y-0 md:scale-100");
										props.set_attribute("x-transition:leave-end", "opacity-0 translate-y-4 md:translate-y-0 md:scale-95");
										props.set_attribute("x-description", "Modal panel, show/hide based on modal state.");
										props.styles(&[Style::Flex, NoStyle::Noop("w-full"), NoStyle::Noop("transform"), NoStyle::Noop("text-left"), NoStyle::Noop("text-base"), NoStyle::Noop("transition"), Screen::Medium(&[Style::MarginY(32)]), Screen::Medium(&[NoStyle::Noop("max-w-2xl")]), Screen::Medium(&[Style::PaddingX(16)]), Screen::Large(&[NoStyle::Noop("max-w-4xl")])]);
										props.set_attribute("@click.away", "open = false");
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[NoStyle::Noop("relative"), Style::Flex, NoStyle::Noop("w-full"), Style::ItemsCenter, NoStyle::Noop("overflow-hidden"), NoStyle::Noop("bg-white"), Style::PaddingX(16), Style::PaddingBottom(32), Style::PaddingTop(56), NoStyle::Noop("shadow-2xl"), Screen::Small(&[Style::PaddingX(24)]), Screen::Small(&[Style::PaddingTop(32)]), Screen::Medium(&[Style::Padding(24)]), Screen::Large(&[Style::Padding(32)])]);
											
											props.child("1", Dynamic).run("button", |props| {
												props.set_attribute("type", "button");
												props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("right-4"), NoStyle::Noop("top-4"), Style::TextColor(Color::Fg(44)), Action::Hover(&[Style::TextColor(Color::Fg(56))]), Screen::Small(&[NoStyle::Noop("right-6")]), Screen::Small(&[NoStyle::Noop("top-8")]), Screen::Medium(&[NoStyle::Noop("right-6")]), Screen::Medium(&[NoStyle::Noop("top-6")]), Screen::Large(&[NoStyle::Noop("right-8")]), Screen::Large(&[NoStyle::Noop("top-8")])]);
												props.set_attribute("@click", "open = false");
												
												props.child("1", Dynamic).run("span", |props| {
													props.styles(&[NoStyle::Noop("sr-only")]);
													
													props.child("0", Label).run(|props| props.set_text("Close"));
												});
												props.child("3", Icon).run(|props| {
													props.style(&[Style::Width(24), Style::Width(24)]);
												});
											});
											props.child("3", Dynamic).run("div", |props| {
												props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("w-full"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("items-start"), NoStyle::Noop("gap-x-6"), NoStyle::Noop("gap-y-8"), Screen::Small(&[NoStyle::Noop("grid-cols-12")]), Screen::Large(&[NoStyle::Noop("gap-x-8")])]);
												
												props.child("1", Dynamic).run("div", |props| {
													props.styles(&[NoStyle::Noop("aspect-h-3"), NoStyle::Noop("aspect-w-2"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(11)), Screen::Small(&[NoStyle::Noop("col-span-4")]), Screen::Large(&[NoStyle::Noop("col-span-5")])]);
													
													props.child("1", Dynamic).run("img", |props| {
														props.set_attribute("src", "https://tailwindui.com/img/ecommerce-images/product-quick-preview-02-detail.jpg");
														props.set_attribute("alt", "Two each of gray, white, and black shirts arranged on table.");
														props.styles(&[NoStyle::Noop("object-cover"), NoStyle::Noop("object-center")]);
													});
												});
												props.child("3", Dynamic).run("div", |props| {
													props.styles(&[Screen::Small(&[NoStyle::Noop("col-span-8")]), Screen::Large(&[NoStyle::Noop("col-span-7")])]);
													
													props.child("1", Dynamic).run("h2", |props| {
														props.styles(&[NoStyle::Noop("text-2xl"), Style::FontBold, Style::TextColor(Color::Fg(100)), Screen::Small(&[Style::PaddingRight(48)])]);
														
														props.child("0", Label).run(|props| props.set_text("Basic Tee 6-Pack"));
													});
													props.child("3", Dynamic).run("section", |props| {
														props.set_attribute("aria-labelledby", "information-heading");
														props.styles(&[Style::MarginTop(8)]);
														
														props.child("1", Dynamic).run("h3", |props| {
															props.set_attribute("id", "information-heading");
															props.styles(&[NoStyle::Noop("sr-only")]);
															
															props.child("0", Label).run(|props| props.set_text("Product information"));
														});
														props.child("3", Dynamic).run("p", |props| {
															props.styles(&[NoStyle::Noop("text-2xl"), Style::TextColor(Color::Fg(100))]);
															
															props.child("0", Label).run(|props| props.set_text("$192"));
														});
														//  Reviews 
														props.child("7", Dynamic).run("div", |props| {
															props.styles(&[Style::MarginTop(24)]);
															
															props.child("1", Dynamic).run("h4", |props| {
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
																	
																	props.child("0", Label).run(|props| props.set_text("3.9 out of 5 stars"));
																});
																props.child("5", Dynamic).run("a", |props| {
																	props.set_attribute("href", "#");
																	props.styles(&[Style::MarginLeft(12), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-indigo-600"), Action::Hover(&[NoStyle::Noop("text-indigo-500")])]);
																	
																	props.child("0", Label).run(|props| props.set_text("117 reviews"));
																});
															});
														});
													});
													props.child("5", Dynamic).run("section", |props| {
														props.set_attribute("aria-labelledby", "options-heading");
														props.styles(&[Style::MarginTop(40)]);
														
														props.child("1", Dynamic).run("h3", |props| {
															props.set_attribute("id", "options-heading");
															props.styles(&[NoStyle::Noop("sr-only")]);
															
															props.child("0", Label).run(|props| props.set_text("Product options"));
														});
														props.child("3", Dynamic).run("form", |props| {
															//  Colors 
															props.child("3", Dynamic).run("div", |props| {
																props.child("1", Dynamic).run("h4", |props| {
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
																	props.child("3", Dynamic).run("span", |props| {
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
																	
																	props.child("1", Dynamic).run("h4", |props| {
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
																		props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-4"), NoStyle::Noop("gap-4")]);
																		
																		props.child("1", Dynamic).run("label", |props| {
																			props.set_attribute("x-radio-group-option", "");
																			props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("bg-white"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm")]);
																			props.set_attribute("x-state:on:size.instock", "In Stock");
																			props.set_attribute("x-state:off:size.instock", "Out of Stock");
																			props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
																			props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'XXS'), 'undefined': !(active === 'XXS') }");
																			
																			props.child("1", Dynamic).run("input", |props| {
																				props.set_attribute("type", "radio");
																				props.set_attribute("x-model", "value");
																				props.set_attribute("name", "size-choice");
																				props.set_attribute("value", "XXS");
																				props.styles(&[NoStyle::Noop("sr-only")]);
																				props.set_attribute("aria-labelledby", "size-choice-0-label");
																			});
																			props.child("3", Dynamic).run("span", |props| {
																				props.set_attribute("id", "size-choice-0-label");
																				
																				props.child("0", Label).run(|props| props.set_text("XXS"));
																			});
																			props.child("5", Dynamic).run("span", |props| {
																				props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("absolute"), NoStyle::Noop("-inset-px"), NoStyle::Noop("rounded-md")]);
																				props.set_attribute("aria-hidden", "true");
																				props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
																				props.set_attribute(":class", "{ 'border': (active === 'XXS'), 'border-2': !(active === 'XXS'), 'border-indigo-500': (value === 'XXS'), 'border-transparent': !(value === 'XXS') }");
																			});
																		});
																		props.child("3", Dynamic).run("label", |props| {
																			props.set_attribute("x-radio-group-option", "");
																			props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("bg-white"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm")]);
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
																			props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("bg-white"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm")]);
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
																			props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("bg-white"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm")]);
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
																			props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("bg-white"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm")]);
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
																			props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("bg-white"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm")]);
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
																			props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("bg-white"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm")]);
																			props.set_attribute("x-state:on:size.instock", "In Stock");
																			props.set_attribute("x-state:off:size.instock", "Out of Stock");
																			props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
																			props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'XXL'), 'undefined': !(active === 'XXL') }");
																			
																			props.child("1", Dynamic).run("input", |props| {
																				props.set_attribute("type", "radio");
																				props.set_attribute("x-model", "value");
																				props.set_attribute("name", "size-choice");
																				props.set_attribute("value", "XXL");
																				props.styles(&[NoStyle::Noop("sr-only")]);
																				props.set_attribute("aria-labelledby", "size-choice-6-label");
																			});
																			props.child("3", Dynamic).run("span", |props| {
																				props.set_attribute("id", "size-choice-6-label");
																				
																				props.child("0", Label).run(|props| props.set_text("XXL"));
																			});
																			props.child("5", Dynamic).run("span", |props| {
																				props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("absolute"), NoStyle::Noop("-inset-px"), NoStyle::Noop("rounded-md")]);
																				props.set_attribute("aria-hidden", "true");
																				props.set_attribute("x-description", r#"Active: "border", Not Active: "border-2"	Checked: "border-indigo-500", Not Checked: "border-transparent""#);
																				props.set_attribute(":class", "{ 'border': (active === 'XXL'), 'border-2': !(active === 'XXL'), 'border-indigo-500': (value === 'XXL'), 'border-transparent': !(value === 'XXL') }");
																			});
																		});
																		props.child("15", Dynamic).run("label", |props| {
																			props.set_attribute("x-radio-group-option", "");
																			props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), Style::PaddingY(12), Style::PaddingX(16), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("uppercase"), Action::Hover(&[Style::Color(Color::Fg(6))]), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("flex-1")]), NoStyle::Noop("cursor-not-allowed"), Style::Color(Color::Fg(6)), Style::TextColor(Color::Fg(22))]);
																			props.set_attribute("x-state:on:size.instock", "In Stock");
																			props.set_attribute("x-state:off:size.instock", "Out of Stock");
																			props.set_attribute("x-description", r#"Active: "ring-2 ring-indigo-500""#);
																			props.set_attribute(":class", "{ 'ring-2 ring-indigo-500': (active === 'XXXL'), 'undefined': !(active === 'XXXL') }");
																			
																			props.child("1", Dynamic).run("input", |props| {
																				props.set_attribute("type", "radio");
																				props.set_attribute("x-model", "value");
																				props.set_attribute("name", "size-choice");
																				props.set_attribute("value", "XXXL");
																				props.set_attribute("disabled", "");
																				props.styles(&[NoStyle::Noop("sr-only")]);
																				props.set_attribute("aria-labelledby", "size-choice-7-label");
																			});
																			props.child("3", Dynamic).run("span", |props| {
																				props.set_attribute("id", "size-choice-7-label");
																				
																				props.child("0", Label).run(|props| props.set_text("XXXL"));
																			});
																			props.child("5", Dynamic).run("span", |props| {
																				props.set_attribute("aria-hidden", "true");
																				props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("absolute"), NoStyle::Noop("-inset-px"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-2"), NoStyle::Noop("border-gray-200")]);
																				
																				props.child("1", Icon).run(|props| {
																					props.style(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0"), NoStyle::Noop("h-full"), NoStyle::Noop("w-full"), NoStyle::Noop("stroke-2"), Style::TextColor(Color::Fg(22))]);
																				});
																			});
																		});
																	});
																});
															});
															props.child("9", Dynamic).run("button", |props| {
																props.set_attribute("type", "submit");
																props.styles(&[Style::MarginTop(24), Style::Flex, NoStyle::Noop("w-full"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("border"), NoStyle::Noop("border-transparent"), NoStyle::Noop("bg-indigo-600"), Style::PaddingX(32), Style::PaddingY(12), NoStyle::Noop("text-base"), NoStyle::Noop("font-medium"), NoStyle::Noop("text-white"), Action::Hover(&[NoStyle::Noop("bg-indigo-700")]), Action::Hover(&[NoStyle::Noop("outline-none")]), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-indigo-500")]), Action::Hover(&[NoStyle::Noop("ring-offset-2")])]);
																
																props.child("0", Label).run(|props| props.set_text("Add to bag"));
															});
														});
													});
												});
											});
										});
									});
								});
							});
						});
					});
				});
			});
		});
	}
}
